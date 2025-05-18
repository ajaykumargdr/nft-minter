use super::Result;
use crate::{blockchain::GTKContract, secret_storage::HcpClient};
use actix_web::{
    App, HttpResponse, HttpServer, Responder, http::StatusCode, middleware::Logger, web,
};
use authentication::AuthenticationGuard;
use std::sync::Mutex;

mod authentication;
mod authorization;
mod marketplace;
mod types;

use types::*;

// Todo - move to db
static USERS: Mutex<Vec<User>> = Mutex::new(Vec::new());

#[actix_web::get("/")]
async fn index(auth_guard: AuthenticationGuard, context: web::Data<ActixContext>) -> String {
    let user = auth_guard.user;

    println!("UserID: {}, Address: {}", user.id, user.wallet_address);

    context.contract.contract_name().await.unwrap()
}

// Todo : get password
#[actix_web::post("/mint")]
async fn mint(
    auth_guard: AuthenticationGuard,
    context: web::Data<ActixContext>,
    input: web::Json<MintInfo>,
) -> impl Responder {
    let user = auth_guard.user;

    println!(
        "minting token id: {} to: {}",
        input.token_id, user.wallet_address
    );

    context
        .contract
        .mint_nft(&user.wallet_address, input.token_id, &input.token_uri)
        .await
        .unwrap();

    HttpResponse::new(StatusCode::OK)
}

#[actix_web::get("/owner/{token_id}")]
async fn owner(
    _auth_guard: AuthenticationGuard,
    context: web::Data<ActixContext>,
    token_id: web::Path<usize>,
) -> impl Responder {
    // Todo : handle errors
    context.contract.owner_of_token(token_id.into_inner()).await
}

#[actix_web::put("/transfer")]
async fn transfer_nft(
    _auth_guard: AuthenticationGuard,
    context: web::Data<ActixContext>,
    input: web::Json<TransferInfo>,
) -> impl Responder {
    // Todo : check owner first

    // Todo : handle errors
    context
        .contract
        .transfer_nft(&input.from, &input.to, input.token_id)
        .await
}

#[actix_web::get("/metadata/{token_id}")]
async fn metadata(
    _auth_guard: AuthenticationGuard,
    context: web::Data<ActixContext>,
    token_id: web::Path<usize>,
) -> impl Responder {
    match context.contract.get_metadata(token_id.into_inner()).await {
        Ok(metadata) => HttpResponse::Ok().json(metadata),
        Err(_) => {
            // Todo : handle errors
            HttpResponse::NotFound().finish()
        }
    }
}

pub async fn start_server() -> Result<()> {
    let contract = GTKContract::new().await.unwrap();

    let client = reqwest::Client::new();
    let client_id = std::env::var("HCP_CLIENT_ID").unwrap();
    let client_secret = std::env::var("HCP_CLIENT_SECRET").unwrap();
    let org_id = std::env::var("HCP_ORG_ID").unwrap();
    let proj_id = std::env::var("HCP_PROJ_ID").unwrap();
    let app_name = std::env::var("HCP_APP_NAME").unwrap();

    let secret_manager =
        HcpClient::new(&client, client_id, client_secret, org_id, proj_id, app_name).await?;

    let context = ActixContext {
        contract,
        http_client: client,
        secret_manager,
    };

    Ok(HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(context.clone()))
            .service(index)
            .service(mint)
            .service(owner)
            .service(transfer_nft)
            .service(metadata)
            .service(marketplace::list)
            .service(marketplace::get_listings)
            .service(marketplace::buy)
            .service(marketplace::update_listing)
            .service(marketplace::cancel_listing)
            .service(authorization::google_oauth_handler)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?)
}
