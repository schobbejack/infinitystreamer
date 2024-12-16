use crate::api_error::ApiError;
use actix_web::http::header;
use actix_web::{get, web, HttpRequest, HttpResponse};

#[get("/master.m3u8")]
async fn get_master(req: HttpRequest) -> Result<HttpResponse, ApiError> {
    let query = req.query_string();

    // TODO: parse codec parameters from fragment
    let header = format!(
        include_str!("../../assets/master.m3u8"),
        query, query, query
    );

    // TODO: get from proxy header
    println!(
        "new playout initiated from {}",
        req.connection_info().peer_addr().unwrap_or("unknown")
    );

    Ok(HttpResponse::Ok()
        .insert_header(header::ContentType(
            "application/vnd.apple.mpegurl".parse().unwrap(),
        ))
        .body(header))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_master);
}
