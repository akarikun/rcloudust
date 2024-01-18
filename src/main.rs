use cookie::time::Duration;
use cookie::Cookie;
use rust_embed::RustEmbed;
use salvo::serve_static::StaticDir;
use salvo::{prelude::*, serve_static::static_embed};
mod models;
mod utils;
use models::accounts::*;
use utils::AppConfig::*;

#[derive(RustEmbed)]
#[folder = "www"]
struct Assets;

#[tokio::main]
async fn main() {
    let cfg = AppConfig::get_config("./config.json").unwrap();
    println!("{:#?}", cfg);
    tracing_subscriber::fmt().init();

    let router = Router::new()
        .push(
            //存放文件上传目录
            Router::with_path("/files/<**path>").get(
                StaticDir::new(["files"])
                    .include_dot_files(false)
                    .auto_list(true),
            ),
        )
        .push(Router::with_path("/api/account/login").post(login))
        // .push(Router::with_path("/set_cookie").get(login))
        // .push(Router::with_path("/get_cookie").get(api_list))
        .push(Router::with_path("/api/<**path>").post(api_list))
        .push(Router::with_path("/404").post(page_404))
        //将www目录中的资源嵌入到可执行文件中
        .push(Router::with_path("<**path>").get(static_embed::<Assets>().fallback("index.html")));

    let acceptor = TcpListener::new(format!("127.0.0.1:{}", cfg.port))
        .bind()
        .await;
    Server::new(acceptor).serve(router).await;
}

#[handler]
async fn api_list(
    _req: &mut Request,
    _depot: &mut Depot,
    res: &mut Response,
    _ctrl: &mut FlowCtrl,
) {
    println!("{:?}", _req.uri());
    // if let Some(name) = _req.cookies().get("name").map(|c| c.value()) {
    //     println!("@@@ {:?}", name);
    // }
    res.render("api_list");
}

#[handler]
async fn login(_req: &mut Request, _depot: &mut Depot, res: &mut Response, _ctrl: &mut FlowCtrl) {
    let host = _req.uri().host().unwrap().to_string();
    if let Ok(ref data) = _req.parse_json::<AccountsModelInput>().await {
        let json = serde_json::to_string_pretty(data).unwrap();
        let cookie: Cookie = Cookie::build(("name", json))
            .domain(host)
            .path("/")
            .secure(true)
            .http_only(true)
            .max_age(Duration::days(30))
            .build();
        res.add_cookie(cookie);
    } else {
        res.render(Redirect::other("/404"));
    }
}

#[handler]
async fn page_404(
    _req: &mut Request,
    _depot: &mut Depot,
    res: &mut Response,
    _ctrl: &mut FlowCtrl,
) {
    res.status_code(StatusCode::NOT_FOUND);
    res.render("404");
}
