use actix_web::{HttpResponse, ResponseError};
use log::error;
use std::fmt;

pub const URL_TO_SAVE: &str = "store";
pub const URL_TO_PREVIEW: &str = "preview";

pub fn create_preview(filename: String) {
    let width = 100;
    let height = 100;
    let img = image::open(format!("{}/{}", URL_TO_SAVE, filename)).unwrap();

    let resized = img.resize(width, height, image::imageops::Nearest);
    &resized.save(format!("{}/{}", URL_TO_PREVIEW, filename));
}

#[derive(Debug)]
pub enum LoadImageError {
    UrlForImageUnreachable,
    InvalidData,
    AsyncIo,
    DecodeBase64,
}

/// Actix web uses `ResponseError` for conversion of errors to a response
impl ResponseError for LoadImageError {
    fn error_response(&self) -> HttpResponse {
        match self {
            LoadImageError::UrlForImageUnreachable => {
                error!("Url for image unreachable by get request");
                HttpResponse::UnprocessableEntity().finish()
            }

            LoadImageError::AsyncIo => {
                error!("IO error during creating file");
                HttpResponse::UnprocessableEntity().finish()
            }

            LoadImageError::InvalidData => {
                error!("Input data is invalid");
                HttpResponse::UnprocessableEntity().finish()
            }

            LoadImageError::DecodeBase64 => {
                error!("Input data is invalid");
                HttpResponse::BadRequest().finish()
            }
        }
    }
}

impl fmt::Display for LoadImageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LoadImageError::UrlForImageUnreachable => {
                write!(f, "Url for image unreachable by get request")
            }
            LoadImageError::AsyncIo => write!(f, "IO error during creating file"),
            LoadImageError::InvalidData => write!(f, "Input data is invalid"),
            LoadImageError::DecodeBase64 => write!(f, "Error during decode base64 data"),
        }
    }
}
