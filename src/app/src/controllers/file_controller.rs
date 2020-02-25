use actix_multipart::Multipart;
use actix_web::{post, Error, HttpResponse};
use async_std::prelude::*;

use std::path::Path;

use crate::mods::helper::{generate_random_string, get_file_ext};
use crate::mods::error::{HttpResponseBuilderExt};
use domain::models::files::File;

#[post("/upload")]
pub async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let mut expires = String::from("");
        let mut key = String::from("");
        let content_disposition = field.content_disposition()
                                       .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;

        if let Some(name) = content_disposition.get_name() {
            match name {
                "MOKO-Expires" => {
                    return Ok(HttpResponse::BadRequest().into())
                },
                _ => {
                    return Ok(HttpResponse::BadRequest().error_response("not allowed any field.",
                                                                        "don't add to field any key-values."))
                }
            }
        }

        //let content_length_value = match field.headers().get("Content_Length") {
        //    Some(value) => { value },
        //    None => { return Ok(HttpResponse::LengthRequired().into()); }
        //};

        //let content_length = content_length_value.to_str().unwrap().parse::<i32>().unwrap_or(0);
        //if content_length > CONTENT_LENGTH_LIMIT { return Ok(HttpResponse::PayloadTooLarge().into()); }

        let orig_filename = content_disposition.get_filename().ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
        let new_file_name = generate_random_string(16);
        let filepath = format!("./tmp/{}.{}", new_file_name, &get_file_ext(&orig_filename));

        if !Path::new("./tmp").exists() { async_std::fs::create_dir("./tmp").await?; }
        let mut f = async_std::fs::File::create(filepath).await?;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).await?;
        }
    }
    Ok(HttpResponse::Ok().into())
}
