use actix_web::{web, Error, HttpResponse};
use reqwest;

use std::io::Write;
use serde::Deserialize;
use actix_web::{error, Result};
use thiserror::Error;

use crate::handlers::lib::{create_preview, URL_TO_SAVE};

#[derive(Deserialize)]
pub struct Info {
    url: String,
}

/// LoadImageError enumerates all possible errors returned by this library.
#[derive(Error, Debug)]
pub enum LoadImageError {
    #[error("first letter must be lowercase but was")]
    BadError, 

    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

// Use default implementation for `error_response()` method
impl error::ResponseError for LoadImageError {}

#[post("/load_by_url")]
pub async fn call(info: web::Query<Info>) -> Result<HttpResponse, Error> {
    let response = reqwest::get(&info.url).await.expect("Error during cal given url");

    let collection = &info.url.split("/").collect::<Vec<&str>>();
    let filename = collection.last().unwrap();
    let file_path = format!("{}/{}", URL_TO_SAVE, &filename);

    let mut f = web::block(|| std::fs::File::create(file_path))
        .await
        .expect("Failed to create file!");

    let data = response.bytes().await.expect("Error during get image data from url");

    web::block(move || f.write_all(&data).map(|_| f)).await?;

    create_preview(filename.to_string());

    Ok(HttpResponse::Ok().into())
}