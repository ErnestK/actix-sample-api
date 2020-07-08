use actix_multipart::Multipart;
use actix_web::{web, Error, HttpResponse};
use serde::Deserialize;
use base64;
use futures::{StreamExt, TryStreamExt};
use std::io::Write;

use crate::handlers::lib::{create_preview, URL_TO_SAVE};

#[derive(Deserialize)]
pub struct Info {
    name: String,
    ext: String
}

#[post("/load_decode_image")]
pub async fn call(mut payload: Multipart, info: web::Query<Info>) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let filename = format!("{}.{}", &info.name, &info.ext);
        let filepath = format!("{}/{}", URL_TO_SAVE, &filename);
        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .expect("Failed to create file!");

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = base64::decode(chunk.unwrap()).unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }

        create_preview(filename);
    }
    
    Ok(HttpResponse::Ok().into())
}
