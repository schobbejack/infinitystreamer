use crate::models::asset::{build_live_manifest, build_vod_manifest, Asset};
use crate::models::fragment::FragmentParams;
use crate::models::query::Query;
use crate::mp4utils::patch_fragment;
use crate::vtt::generate;
use actix_web::{get, http::header, web, HttpResponse};

use crate::api_error::ApiError;

#[get("/media/{track}/index.m3u8")]
async fn get_media_track(
    asset: web::Data<Asset>,
    path: web::Path<String>,
    query: web::Query<Query>,
) -> Result<HttpResponse, ApiError> {
    let asset = asset.into_inner();
    let track = path.into_inner();

    match track.as_str() {
        "video" => get_media_playlist(&query.into_inner(), &asset.video, false),
        "audio" => get_media_playlist(&query.into_inner(), &asset.audio, false),
        _ => Err(ApiError::new(404, "Track not found")),
    }
}

#[get("/vtt/index.m3u8")]
async fn get_vtt_track(
    asset: web::Data<Asset>,
    query: web::Query<Query>,
) -> Result<HttpResponse, ApiError> {
    let asset = asset.into_inner();
    get_media_playlist(&query.into_inner(), &asset.video, true)
}

#[get("/vtt/{sequence}.fragment")]
async fn get_vtt_fragment(
    asset: web::Data<Asset>,
    path: web::Path<i64>,
) -> Result<HttpResponse, ApiError> {
    let asset = asset.into_inner();
    let sequence = path.into_inner();

    Ok(HttpResponse::Ok()
        .insert_header(header::ContentType("text/vtt".parse().unwrap()))
        .body(generate(sequence, &asset.video)?))
}

#[get("/media/{track}/init.m4i")]
async fn get_media_init(
    asset: web::Data<Asset>,
    path: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let asset = asset.into_inner();
    let track = path.into_inner();

    let blob: &[u8] = match track.as_str() {
        "video" => Ok(asset.video.init.as_ref()),
        "audio" => Ok(asset.audio.init.as_ref()),
        _ => Err(ApiError::new(404, "Track not found")),
    }?;

    Ok(HttpResponse::Ok()
        .insert_header(header::ContentType("video/mp4".parse().unwrap()))
        .body(&blob[..]))
}

#[get("/media/{track}/{sequence}.fragment")]
async fn get_media_fragment(
    asset: web::Data<Asset>,
    path: web::Path<(String, i64)>,
) -> Result<HttpResponse, ApiError> {
    let asset = asset.into_inner();
    let (track, sequence) = path.into_inner();

    let blob = match track.as_str() {
        "video" => {
            let mut patched_fragment = asset.video.fragment.to_vec();
            patch_fragment(&mut patched_fragment, sequence, asset.video.duration as i64)?;
            Ok(patched_fragment)
        }
        "audio" => {
            let mut patched_fragment = asset.audio.fragment.to_vec();
            patch_fragment(&mut patched_fragment, sequence, asset.audio.duration as i64)?;
            Ok(patched_fragment)
        }
        _ => Err(ApiError::new(404, "Fragment not found")),
    }?;

    Ok(HttpResponse::Ok()
        .insert_header(header::ContentType("video/mp4".parse().unwrap()))
        .body(blob))
}

fn get_media_playlist<const N: usize, const M: usize>(
    query: &Query,
    fragment: &FragmentParams<N, M>,
    is_vtt: bool,
) -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok()
        .insert_header(header::ContentType(
            "application/vnd.apple.mpegurl".parse().unwrap(),
        ))
        .body(match query.start {
            Some(..) => build_vod_manifest(&query, fragment, is_vtt)?,
            None => build_live_manifest(fragment, is_vtt)?,
        }))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_media_track);
    cfg.service(get_vtt_track);
    cfg.service(get_vtt_fragment);
    cfg.service(get_media_init);
    cfg.service(get_media_fragment);
}
