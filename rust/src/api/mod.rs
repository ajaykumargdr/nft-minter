use crate::blockchain::GTKContract;
use actix_web::{web, App, HttpServer};

#[actix_web::get("/")]
async fn index(contract: web::Data<GTKContract>) -> String {
    contract.contract_name().await.unwrap()
}

pub async fn start_server() -> std::io::Result<()> {
    let contract = GTKContract::new().await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(contract.clone()))
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
