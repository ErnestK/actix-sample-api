use actix_multipart::Multipart;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use base64;
use futures::{StreamExt, TryStreamExt};
use std::io::Write;

use crate::handlers::lib::{create_preview, URL_TO_SAVE, LoadImageError};

#[derive(Deserialize)]
pub struct Info {
    name: String
}

#[post("/load_decode_image")]
pub async fn call(mut payload: Multipart, info: web::Query<Info>) -> Result<HttpResponse, LoadImageError> {
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let filepath = format!("{}/{}", URL_TO_SAVE, &info.name);
        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .map_err(|_| LoadImageError::AsyncIo)?;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = base64::decode(chunk.map_err(|_| LoadImageError::AsyncIo)?)
                .map_err(|_| LoadImageError::DecodeBase64)?;
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f)).await.map_err(|_| LoadImageError::AsyncIo)?;
        }

        create_preview(info.name.to_string());
    }
    
    Ok(HttpResponse::Ok().into())
}
