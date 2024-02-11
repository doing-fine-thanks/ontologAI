use sha2::{Digest, Sha256};
use actix_web::HttpResponse;

pub fn generate_hash_key(key: &String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key);
    format!("{:x}", hasher.finalize())
}


pub fn not_found() -> HttpResponse {
    HttpResponse::NotFound().content_type("text/html; charset=utf-8").body("<h1>404 Not Found</h1>")
}

pub fn interal_error() -> HttpResponse {
    HttpResponse::InternalServerError().content_type("text/html; charset=utf-8").body("<h1>500 Not Found</h1>")
}
