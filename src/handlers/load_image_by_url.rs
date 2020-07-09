use actix_web::{web, HttpResponse, Result};
use reqwest;
use std::io::Write;
use serde::Deserialize;

use crate::handlers::lib::{create_preview, URL_TO_SAVE, LoadImageError};

#[derive(Deserialize)]
pub struct Info {
    url: String,
}

pub async fn call(info: web::Query<Info>) -> Result<HttpResponse, LoadImageError> {
    let response = reqwest::get(&info.url).await.map_err(|_| LoadImageError::UrlForImageUnreachable)?;

    let collection = &info.url.split("/").collect::<Vec<&str>>();
    let filename = collection.last();
    if filename.is_none() {
        return Err(LoadImageError::InvalidData);
    }
    let filename = filename.unwrap();
    let file_path = format!("{}/{}", URL_TO_SAVE, &filename);

    let mut f = web::block(|| std::fs::File::create(file_path))
        .await
        .map_err(|_| LoadImageError::AsyncIo)?;

    let data = response.bytes().await.expect("Error during get image data from url");

    web::block(move || f.write_all(&data).map(|_| f)).await.map_err(|_| LoadImageError::AsyncIo)?;

    create_preview(filename.to_string());

    Ok(HttpResponse::Ok().into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};
    use actix_web::http::StatusCode;
    use std::fs;
    use std::path::Path;
    use crate::handlers::lib::{URL_TO_SAVE, URL_TO_PREVIEW};

    #[actix_rt::test]
    async fn test_load_image_by_url() {
        let root_uri = "/";
        let filename = "rust-logo-512x512.png";
        let filepath = format!("{}/{}", URL_TO_SAVE, filename);
        let preview_filepath = format!("{}/{}", URL_TO_PREVIEW, filename);
        let params = format!("url=https://www.rust-lang.org/logos/{}", filename);
        let uri = format!("{}?{}", root_uri, params);
        let mut app = test::init_service(App::new().route(root_uri, web::get().to(call))).await;
        let req = test::TestRequest::with_uri(&uri).to_request();

        // TODO: is this bad that service do call to net and fail if cant, "yes" from one side and "no" from other
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
        assert!(Path::new(&filepath).exists());
        assert!(Path::new(&preview_filepath).exists());

        fs::remove_file(filepath).expect("Failed during remove test image");
        fs::remove_file(preview_filepath).expect("Failed during remove preview test image");
    }
}