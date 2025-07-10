mod config;
mod entity;
mod handlers;
mod models;
mod routes;
mod utils;
use crate::config::settings::{DatabaseConfig, Settings};
use actix_web::{App, HttpResponse, HttpServer, Responder, get, HttpRequest, Error, web};
use std::path::PathBuf;
#[cfg(not(debug_assertions))]
use rust_embed::RustEmbed;
use mime;
#[cfg(not(debug_assertions))]
use mime_guess::from_path;

// 简单路由
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello Rust Web!")
}
#[macro_use]
extern crate simple_log;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = Settings::load_config();

    if let Ok(config) = settings {
        //初始化日志
        let _log = simple_log::new(config.log_config);
        //初始化数据库
        let _db = DatabaseConfig::init_database(config.database_config).await;

        let server =
            HttpServer::new(move || {
                App::new()
                    .configure(routes::path_exposure::path_config)
                    .service(web::resource("/{path:.*}").to(handle_request))
            })
                .bind(config.web_config.format());

        if let Ok(httpserver) = server {
            let _ = httpserver.run().await;
        }
    }

    Ok(())
}

#[derive(RustEmbed)]
#[folder = "frontend/dist"]
#[cfg(not(debug_assertions))]
struct WebAssets;

#[cfg(debug_assertions)]
const WEB_ENTRY: &str = "frontend/dist";
#[cfg(debug_assertions)]
const WEB_INDEX: &str = "frontend/dist/index.html";
async fn handle_request(req: HttpRequest) -> Result<HttpResponse, Error> {
    let path = req.match_info().query("path");
    let path_str = path.trim_start_matches('/');

    // 判断是否是文件请求（有扩展名）
    let is_file_request = path_str.contains('.');

    // 文件请求处理
    if is_file_request {
        // 开发环境：从文件系统加载
        #[cfg(debug_assertions)]
        {
            let fs_path = PathBuf::from(WEB_ENTRY).join(path_str);
            if fs_path.is_file() {
                let file = actix_files::NamedFile::open_async(fs_path).await?;
                return Ok(file.into_response(&req));
            }
        }

        // 生产环境：从嵌入资源加载
        #[cfg(not(debug_assertions))]
        {
            if let Some(asset) = WebAssets::get(path_str) {
                let etag = format!("\"{}\"", hex::encode(asset.metadata.sha256_hash()));

                // 智能MIME类型检测
                let mime = match path_str.rsplit('.').next() {
                    Some("js") => mime::APPLICATION_JAVASCRIPT,
                    Some("css") => mime::TEXT_CSS,
                    Some("html") => mime::TEXT_HTML,
                    Some("json") => mime::APPLICATION_JSON,
                    Some("png") => mime::IMAGE_PNG,
                    Some("jpg") | Some("jpeg") => mime::IMAGE_JPEG,
                    Some("svg") => mime::IMAGE_SVG,
                    _ => from_path(path_str).first_or_octet_stream(),
                };

                // 统一缓存策略
                let cache_policy = if path_str == "index.html" {
                    "no-cache"
                } else {
                    "public, max-age=31536000"
                };

                return Ok(HttpResponse::Ok()
                    .content_type(mime.as_ref())
                    .insert_header(("Cache-Control", cache_policy))
                    .insert_header(("ETag", etag))
                    .body(asset.data));
            }
        }
    }

    // 非文件请求（前端路由）返回 index.html
    handle_index(&req).await
}

// 处理前端路由（返回 index.html）
#[allow(unused_variables)]
async fn handle_index(req: &HttpRequest) -> Result<HttpResponse, Error> {
    // 开发环境：从文件系统加载 index.html
    #[cfg(debug_assertions)]
    {
        let index_path = PathBuf::from(WEB_INDEX);
        if index_path.exists() {
            let file = actix_files::NamedFile::open_async(index_path).await?;
            return Ok(file.into_response(req));
        }
    }

    // 生产环境：从嵌入资源加载 index.html
    #[cfg(not(debug_assertions))]
    {
        if let Some(asset) = WebAssets::get("index.html") {
            let etag = format!("\"{}\"", hex::encode(asset.metadata.sha256_hash()));

            return Ok(HttpResponse::Ok()
                .content_type("text/html")
                .insert_header(("Cache-Control", "no-cache"))
                .insert_header(("ETag", etag))
                .body(asset.data));
        }
    }

    // 如果连 index.html 都没有，返回 404
    Ok(HttpResponse::NotFound().body("404 Not Found"))
}
