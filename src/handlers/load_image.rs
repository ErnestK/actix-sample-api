extern crate image;

use actix_multipart::Multipart;
use actix_web::{web, Error, HttpResponse,};
use futures::{StreamExt, TryStreamExt};
use std::io::Write;

use crate::handlers::lib::{create_preview, URL_TO_SAVE};

pub async fn call(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let filepath = format!("{}/{}", URL_TO_SAVE, sanitize_filename::sanitize(&filename));
        let mut f = std::fs::File::create(&filepath).unwrap();

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }

        create_preview(filename.to_string());
    }
    
    Ok(HttpResponse::Ok().into())
}
