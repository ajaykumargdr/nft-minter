use crate::blockchain::GTKContract;
use actix_web::{App, HttpResponse, HttpServer, Responder, http::StatusCode, web};

mod types;
use types::*;

#[actix_web::get("/")]
async fn index(contract: web::Data<GTKContract>) -> String {
    contract.contract_name().await.unwrap()
}

#[actix_web::post("/mint")]
async fn mint(contract: web::Data<GTKContract>, input: web::Json<MintInfo>) -> impl Responder {
    println!("minting token id: {} to: {}", input.token_id, input.to);

    contract
        .mint_nft(&input.to, input.token_id, &input.token_uri)
        .await
        .unwrap();

    HttpResponse::new(StatusCode::OK)
}

#[actix_web::get("/owner/{token_id}")]
async fn owner(contract: web::Data<GTKContract>, token_id: web::Path<usize>) -> impl Responder {
    // Todo : handle errors
    contract.owner_of_token(token_id.into_inner()).await
}

#[actix_web::put("/transfer")]
async fn transfer_nft(
    contract: web::Data<GTKContract>,
    input: web::Json<TransferInfo>,
) -> impl Responder {
    // Todo : handle errors
    contract
        .transfer_nft(&input.from, &input.to, input.token_id)
        .await
}

pub async fn start_server() -> std::io::Result<()> {
    let contract = GTKContract::new().await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(contract.clone()))
            .service(index)
            .service(mint)
            .service(owner)
            .service(transfer_nft)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
