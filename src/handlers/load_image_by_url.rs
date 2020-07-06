use actix_web::{web, Error, HttpResponse};
use reqwest;

use std::io::Write;
use serde::Deserialize;
use crate::handlers::lib::{create_preview, URL_TO_SAVE};

#[derive(Deserialize)]
pub struct Info {
    url: String,
}

#[post("/load_by_url")]
pub async fn call(info: web::Query<Info>) -> Result<HttpResponse, Error> {
    let response = reqwest::get(&info.url).await.expect("Error during cal given url");

    let filename = &response
        .url()
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or("tmp.tmp");
    
    let file_path = format!("{}/{}", URL_TO_SAVE, &filename);

    // TODO: remove that
    let response1 = reqwest::get(&info.url).await.expect("Error during cal given url");
    {    
        let mut f = web::block(|| std::fs::File::create(file_path))
            .await
            .expect("Failed to create file!");

        let data = response1.bytes().await.expect("Error during get image data from url");

        web::block(move || f.write_all(&data).map(|_| f)).await?;
    };

    create_preview(filename.to_string());

    Ok(HttpResponse::Ok().into())
}