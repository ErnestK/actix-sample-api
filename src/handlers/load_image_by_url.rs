use actix_web::{web, HttpResponse, Result};
use reqwest;
use std::io::Write;
use serde::Deserialize;

use crate::handlers::lib::{create_preview, URL_TO_SAVE, LoadImageError};

#[derive(Deserialize)]
pub struct Info {
    url: String,
}

#[post("/load_by_url")]
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