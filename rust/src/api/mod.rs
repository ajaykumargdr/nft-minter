use super::Result;
use crate::blockchain::GTKContract;
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
async fn index(auth_guard: AuthenticationGuard, contract: web::Data<GTKContract>) -> String {
    println!("UserID: {}", auth_guard.user_id);
    contract.contract_name().await.unwrap()
}

#[actix_web::post("/mint")]
async fn mint(
    _auth_guard: AuthenticationGuard,
    contract: web::Data<GTKContract>,
    input: web::Json<MintInfo>,
) -> impl Responder {
    println!("minting token id: {} to: {}", input.token_id, input.to);

    contract
        .mint_nft(&input.to, input.token_id, &input.token_uri)
        .await
        .unwrap();

    HttpResponse::new(StatusCode::OK)
}

#[actix_web::get("/owner/{token_id}")]
async fn owner(
    _auth_guard: AuthenticationGuard,
    contract: web::Data<GTKContract>,
    token_id: web::Path<usize>,
) -> impl Responder {
    // Todo : handle errors
    contract.owner_of_token(token_id.into_inner()).await
}

#[actix_web::put("/transfer")]
async fn transfer_nft(
    _auth_guard: AuthenticationGuard,
    contract: web::Data<GTKContract>,
    input: web::Json<TransferInfo>,
) -> impl Responder {
    // Todo : handle errors
    contract
        .transfer_nft(&input.from, &input.to, input.token_id)
        .await
}

#[actix_web::get("/metadata/{token_id}")]
async fn metadata(
    _auth_guard: AuthenticationGuard,
    contract: web::Data<GTKContract>,
    token_id: web::Path<usize>,
) -> impl Responder {
    match contract.get_metadata(token_id.into_inner()).await {
        Ok(metadata) => HttpResponse::Ok().json(metadata),
        Err(_) => {
            // Todo : handle errors
            HttpResponse::NotFound().finish()
        }
    }
}

pub async fn start_server() -> std::io::Result<()> {
    let contract = GTKContract::new().await.unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(contract.clone()))
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
    .await
}
