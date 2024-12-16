use std::cmp::min;

use chrono::DateTime;

use super::{fragment::FragmentParams, query::Query};

use crate::{
    api_error::ApiError,
    resources::{
        AUDIO_FRAGMENT_BLOB, AUDIO_FRAGMENT_LENGTH, AUDIO_INIT_BLOB, AUDIO_INIT_LENGTH,
        VIDEO_FRAGMENT_BLOB, VIDEO_FRAGMENT_LENGTH, VIDEO_INIT_BLOB, VIDEO_INIT_LENGTH,
    },
    utility::now_in_timescale,
};

pub struct Asset {
    pub video: FragmentParams<VIDEO_INIT_LENGTH, VIDEO_FRAGMENT_LENGTH>,
    pub audio: FragmentParams<AUDIO_INIT_LENGTH, AUDIO_FRAGMENT_LENGTH>,
}

impl Asset {
    pub fn new() -> Result<Asset, ApiError> {
        Ok(Asset {
            video: FragmentParams::new(VIDEO_INIT_BLOB, VIDEO_FRAGMENT_BLOB)?,
            audio: FragmentParams::new(AUDIO_INIT_BLOB, AUDIO_FRAGMENT_BLOB)?,
        })
    }
}

const LIVE_FRAGMENT_COUNT: i64 = 8;
const MAX_VOD_FRAGMENT_COUNT: i64 = 65536;

fn create_fragment_list(
    sequence: i64,
    fragment_count: i64,
    extinf: f32,
) -> Result<String, ApiError> {
    let mut fragments = String::new();

    for n in 0..fragment_count {
        fragments += &format!(
            "#EXTINF:{},no-desc\r\n{}.fragment\r\n",
            extinf,
            sequence + n
        );
    }
    Ok(fragments)
}

pub fn build_live_manifest<const N: usize, const M: usize>(
    fragment: &FragmentParams<N, M>,
    is_vtt: bool,
) -> Result<String, ApiError> {
    let end_since_epoch = now_in_timescale(fragment.timescale as i64);
    let sequence = end_since_epoch / fragment.duration as i64 - LIVE_FRAGMENT_COUNT;
    let pdt = DateTime::from_timestamp_millis(
        sequence * fragment.duration as i64 * 1000 / fragment.timescale as i64,
    )
    .unwrap()
    .to_rfc3339();

    let header = format!(
        include_str!("../../assets/media/template.m3u8"),
        sequence,
        pdt,
        match is_vtt {
            true => "",
            false => "\n#EXT-X-MAP:URI=\"init.m4i\"",
        }
    );
    let fragments = create_fragment_list(sequence, LIVE_FRAGMENT_COUNT, fragment.extinf)?;

    Ok(header + &fragments)
}

pub fn build_vod_manifest<const N: usize, const M: usize>(
    query: &Query,
    fragment: &FragmentParams<N, M>,
    is_vtt: bool,
) -> Result<String, ApiError> {
    let (start_since_epoch, end_since_epoch, playlist_type) =
        get_playlist_parameters(&query, fragment)?;

    let fragment_count = min(
        (end_since_epoch - start_since_epoch + fragment.duration as i64 - 1)
            / fragment.duration as i64,
        MAX_VOD_FRAGMENT_COUNT,
    );

    let sequence = start_since_epoch / fragment.duration as i64;
    let pdt = DateTime::from_timestamp_millis(sequence * fragment.duration as i64)
        .unwrap()
        .to_rfc3339();

    let header = format!(
        include_str!("../../assets/media/template_vod.m3u8"),
        sequence, playlist_type, pdt, is_vtt
    );
    let mut fragments = create_fragment_list(sequence, fragment_count, fragment.extinf)?;
    if playlist_type == "VOD" {
        fragments += "#EXT-X-ENDLIST\r\n";
    }

    Ok(header + &fragments)
}

fn get_playlist_parameters<const N: usize, const M: usize>(
    query: &Query,
    fragment: &FragmentParams<N, M>,
) -> Result<(i64, i64, &'static str), ApiError> {
    let start = DateTime::parse_from_rfc3339(&query.start.as_ref().unwrap())?;
    let (end, playlist_type) = match query.end.as_ref() {
        Some(v) => (Some(DateTime::parse_from_rfc3339(&v)?), "VOD"),
        None => (None, "EVENT"),
    };

    let start_since_epoch = start.to_utc().timestamp_millis() * fragment.timescale as i64 / 1000;
    let end_since_epoch = match end {
        Some(end) => end.to_utc().timestamp_millis() * fragment.timescale as i64 / 1000,
        None => now_in_timescale(fragment.timescale as i64),
    };

    if end_since_epoch < start_since_epoch {
        return Err(ApiError::new(400, "end < start"));
    }
    Ok((start_since_epoch, end_since_epoch, playlist_type))
}
