#[macro_use]
extern crate actix_web;

use std::{env, io};

use actix_files as fs;
use actix_session::CookieSession;
use actix_web::http::StatusCode;
use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer, Result,};

mod handlers;

/// favicon handler
#[get("/favicon")]
async fn favicon() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/favicon.ico")?)
}

/// 404 handler
async fn p404() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}

async fn welcome(req: HttpRequest) -> Result<HttpResponse> {
    println!("{:?}", req);

    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/welcome.html")))
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    std::fs::create_dir_all("./store").unwrap();
    std::fs::create_dir_all("./preview").unwrap();
    let host = "127.0.0.1:8080";
    
    HttpServer::new(|| {
        App::new()
            // cookie session middleware
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            .data(web::PayloadConfig::new(1 << 25))
            // register favicon
            .service(favicon)
            .service(
                web::resource("/")
                    .route(web::get().to(welcome))
                    .route(web::post().to(handlers::save_file::call)),
            )
            .service(
                web::resource("/encode_image") // TODO: rename route
                .route(
                    web::post().to(handlers::save_encode_image::call)
                )
            )
            .service(handlers::load_image_by_url::call)
            // static files
            .service(fs::Files::new("/preview", "preview").show_files_listing())
            // static files
            .service(fs::Files::new("/store", "store").show_files_listing())
            // default
            .default_service(
                // 404 for GET request
                web::resource("").route(web::get().to(p404))
            )
    })
    .bind(host)?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_test() {
        assert!(true);
    }
}