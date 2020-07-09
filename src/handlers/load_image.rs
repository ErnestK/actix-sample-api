extern crate image;

use actix_multipart::Multipart;
use actix_web::{web, HttpResponse,};
use futures::{StreamExt, TryStreamExt};
use std::io::Write;

use crate::handlers::lib::{create_preview, URL_TO_SAVE, LoadImageError};

pub async fn call(mut payload: Multipart) -> Result<HttpResponse, LoadImageError> {
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().ok_or_else(|| (LoadImageError::InvalidData))?;
        let filename = content_type.get_filename().ok_or_else(|| (LoadImageError::InvalidData))?;
        let filepath = format!("{}/{}", URL_TO_SAVE, sanitize_filename::sanitize(&filename));

        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .map_err(|_| LoadImageError::AsyncIo)?;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.map_err(|_| LoadImageError::AsyncIo)?;
            f = web::block(move || f.write_all(&data).map(|_| f)).await.map_err(|_| LoadImageError::AsyncIo)?;
        }

        create_preview(filename.to_string());
    }
    
    Ok(HttpResponse::Ok().into())
}
