mod api;
mod blockchain;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    api::start_server().await?;

    Ok(())
}
