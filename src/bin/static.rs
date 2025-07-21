use lambda_http::{Error, tracing};
use tower_http::services::ServeDir;

// hacky bullshit
// this should never ship to prod
// only avaliable with --debug --features static
// please for the love of god do not ship this

#[tokio::main]
#[cfg(debug_assertions)]
#[cfg(feature = "static")]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    lambda_http::run(ServeDir::new(".")).await?;
    Ok(())
}
