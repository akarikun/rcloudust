use cookie::time::Duration;
use cookie::Cookie;
use models::TModel::TModel;
use rust_embed::RustEmbed;
use salvo::serve_static::StaticDir;
use salvo::{prelude::*, serve_static::static_embed};
use std::path::Path;
mod models;
mod utils;
use models::{accounts::*, files::*};
use utils::{common::*, AppConfig::*, JsonResp::*};

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
        .push(Router::with_path("/api/upload").post(upload))
        .push(Router::with_path("/api/account/login").post(login))
        // .push(Router::with_path("/set_cookie").get(login))
        // .push(Router::with_path("/get_cookie").get(api_list))
        .push(Router::with_path("/api/<**path>").post(api_list))
        .push(Router::with_path("/404").post(page_404))
        //将www目录中的资源嵌入到可执行文件中
        .push(Router::with_path("<**path>").get(static_embed::<Assets>().fallback("index.html")));

    let acceptor = TcpListener::new(&cfg.host).bind().await;
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
async fn upload(_req: &mut Request, _depot: &mut Depot, res: &mut Response, _ctrl: &mut FlowCtrl) {
    let fs = _req.files("file").await;
    if let Some(fs) = fs {
        let mut msgs = Vec::with_capacity(fs.len());
        for file in fs {
            let rdm = common::get_random(12);
            let file_name = format!("{}_{}", rdm, file.name().unwrap_or("file"));
            let dest = format!("files/{}_{}", rdm, file_name);
            println!("dest:{}", dest);
            if let Err(e) = std::fs::copy(&file.path(), Path::new(&dest)) {
                res.render(Json(JsonResp {
                    status: 0,
                    data: Some(e.to_string()),
                }));
                return;
            }
            let mut input = FilesModelInput {
                id: None,
                display_name: file_name.clone(),
                file_name,
            };
            if let Ok(_) = FilesModel::insert(&mut input) {
                msgs.push(dest);
            } else {
                panic!("上传文件异常");
            }
        }
        res.render(Json(JsonResp {
            status: 1,
            data: Some(msgs.join("\r\n")),
        }));
        return;
    } else {
        res.render(Json(JsonResp {
            status: 0,
            data: Some("上传异常".to_string()),
        }));
    }
}

#[handler]
async fn login(_req: &mut Request, _depot: &mut Depot, res: &mut Response, _ctrl: &mut FlowCtrl) {
    let host = _req.uri().host().unwrap().to_string();
    if let Ok(ref mut data) = _req.parse_json::<AccountsModelInput>().await {
        if let Ok(models) = models::accounts::AccountsModel::get_list(data) {
            // println!("{:?}", res);
            let json = serde_json::to_string_pretty(&models).unwrap();
            let cookie: Cookie = Cookie::build(("name", json))
                .domain(host)
                .path("/")
                .secure(true)
                .http_only(true)
                .max_age(Duration::days(30))
                .build();
            res.add_cookie(cookie);
            return;
        }
    }
    res.render(Redirect::other("/404"));
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
