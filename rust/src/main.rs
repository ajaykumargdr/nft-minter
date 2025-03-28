mod api;
mod blockchain;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    api::start_server().await?;

    Ok(())
}
