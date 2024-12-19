use crate::api_error::ApiError;
use crate::models::asset::Asset;
use crate::models::fragment::FragmentParams;
use actix_web::http::header;
use actix_web::{get, web, HttpRequest, HttpResponse};

#[get("/master.m3u8")]
async fn get_master(asset: web::Data<Asset>, req: HttpRequest) -> Result<HttpResponse, ApiError> {
    let asset = asset.into_inner();
    let query = req.query_string();

    let header = format!(
        include_str!("../../assets/master.m3u8"),
        query,
        query,
        asset.video.bitrate() + asset.audio.bitrate(),
        asset.video.dimension,
        framerate_from_fragment(&asset.video),
        query
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

fn framerate_from_fragment<const N: usize, const M: usize>(fragment: &FragmentParams<N, M>) -> f32 {
    (fragment.sample_count * fragment.timescale) as f32 / fragment.duration as f32
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_master);
}
