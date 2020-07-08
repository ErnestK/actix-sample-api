#[macro_use]
extern crate actix_web;

use std::{env, io};

use actix_files as fs;
use actix_session::CookieSession;
use actix_web::http::StatusCode;
use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer, Result,};

mod handlers;

// TODO:
// error handlers
// dotenv
// vector in params

// tests
// documentation in code
// readme( main documentation )
// use openCV for test ffi usage

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
    let host = "0.0.0.0:8088";
    
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
                    .route(web::post().to(handlers::load_image::call)),
            )
            .service(handlers::load_decode_image::call)
            .service(handlers::load_image_by_url::call)
            // static files
            .service(fs::Files::new("/preview", "preview").show_files_listing())
            .service(fs::Files::new("/store", "store").show_files_listing())
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
    use super::*;
    use actix_web::test;

    #[actix_rt::test]
    async fn test_welcome_ok() {
        let req = test::TestRequest::default().to_http_request();
        let resp = welcome(req).await;
        assert_eq!(resp.unwrap().status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_p404_not_found() {
        let mut app = test::init_service(App::new().route("/not_found", web::get().to(p404))).await;
        let req = test::TestRequest::default().to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
}