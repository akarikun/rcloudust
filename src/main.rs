use cookie::time::Duration;
use cookie::Cookie;
use rust_embed::RustEmbed;
use salvo::serve_static::StaticDir;
use salvo::{prelude::*, serve_static::static_embed};
mod models;
mod utils;
use models::*;
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
    let uri = _req.uri();
    let host = uri.host().unwrap();
    println!("\t\t\t\t{:#?},{:?},{:?}", uri, host, uri.path());

    // let cookie: Cookie = Cookie::build(("name", "value"))
    //     .domain(host.to_string())
    //     .path("/")
    //     .secure(true)
    //     .http_only(true)
    //     .max_age(Duration::days(30))
    //     .build();
    // res.add_cookie(cookie);
    res.render(Redirect::other("/get_cookie"));
}
