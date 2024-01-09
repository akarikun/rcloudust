use rust_embed::RustEmbed;
use salvo::serve_static::StaticDir;
use salvo::{prelude::*, serve_static::static_embed};
mod utils;
use utils::AppConfig::AppConfig;

#[derive(RustEmbed)]
#[folder = "www"]
struct Assets;

#[tokio::main]
async fn main() {
    let cfg = AppConfig::get_config("./config.json").unwrap();
    println!("{:#?}", cfg);
    tracing_subscriber::fmt().init();

    let router = Router::new()
        .push(Router::with_path("<**path>").get(static_embed::<Assets>().fallback("index.html")));

    let acceptor = TcpListener::new(format!("localhost:{}", cfg.port))
        .bind()
        .await;
    Server::new(acceptor).serve(router).await;
}
