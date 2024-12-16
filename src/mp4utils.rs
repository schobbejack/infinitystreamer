use std::ops::Range;

use crate::api_error::ApiError;

pub fn get_box_range(
    blob: &[u8],
    path: &str,
    blob_offset: usize,
) -> Result<std::ops::Range<usize>, ApiError> {
    const MINIMUM_PATH_LENGTH: usize = 5;
    const MINIMUM_BOX_LENGTH: usize = 8;

    // TODO: remove magic numbers
    if path.len() < MINIMUM_PATH_LENGTH || (blob.len() - blob_offset) < MINIMUM_BOX_LENGTH {
        return Err(ApiError::new(500, "box not found"));
    }

    let inner_blob = &blob[blob_offset..];

    let root = path[1..5].as_bytes();
    let found = root == &inner_blob[4..8];

    let mut size: u64 = u32::from_be_bytes(inner_blob[0..4].try_into().unwrap()) as u64;

    let mut box_header_size = MINIMUM_BOX_LENGTH as u64;
    if size == 1 {
        if inner_blob.len() < 16 {
            return Err(ApiError::new(500, "malformed box"));
        }
        box_header_size += 8;
        size = u64::from_be_bytes(inner_blob[8..16].try_into().unwrap());
    }

    if found {
        if path.len() > MINIMUM_PATH_LENGTH {
            return get_box_range(
                &blob,
                &path[MINIMUM_PATH_LENGTH..],
                blob_offset + box_header_size as usize,
            );
        }
        return Ok(std::ops::Range::<usize> {
            start: blob_offset as usize,
            end: (blob_offset + size as usize) as usize,
        });
    }
    get_box_range(blob, &path, blob_offset + size as usize)
}

pub fn patch_fragment(blob: &mut [u8], sequence: i64, duration: i64) -> Result<&[u8], ApiError> {
    const SEQUENCE_RANGE: Range<usize> = 12..16;
    const MEDIA_DECODE_TIME_RANGEV0: Range<usize> = 12..16;
    const MEDIA_DECODE_TIME_RANGEV1: Range<usize> = 12..20;

    let box_range = get_box_range(blob, "/moof/mfhd", 0)?;
    let mfhd = &mut blob[box_range];
    mfhd[SEQUENCE_RANGE].clone_from_slice(&(sequence as u32).to_be_bytes());

    let box_range = get_box_range(blob, "/moof/traf/tfdt", 0)?;
    let tfdt: &mut [u8] = &mut blob[box_range];
    let version = tfdt[8];

    match version {
        0 => Ok(tfdt[MEDIA_DECODE_TIME_RANGEV0]
            .clone_from_slice(&((sequence * duration) as u32).to_be_bytes())),
        1 => {
            Ok(tfdt[MEDIA_DECODE_TIME_RANGEV1]
                .clone_from_slice(&(sequence * duration).to_be_bytes()))
        }
        _ => Err(ApiError::new(500, "Invalid tfdt version")),
    }?;

    Ok(blob)
}

pub fn get_timescale(blob: &[u8]) -> Result<u32, ApiError> {
    let box_range = get_box_range(blob, "/moov/trak/mdia/mdhd", 0)?;
    let mdhd = &blob[box_range];
    let version = mdhd[8];
    let timescale_offset = match version {
        1 => 12 + 16,
        _ => 12 + 8,
    };
    Ok(u32::from_be_bytes(
        mdhd[timescale_offset..timescale_offset + 4]
            .try_into()
            .unwrap(),
    ))
}

pub fn get_fragment_duration(blob: &[u8]) -> Result<u32, ApiError> {
    const REFERENCE_SIZE: usize = 12;
    const DURATION_OFFSET: usize = 4;
    let box_range = get_box_range(blob, "/sidx", 0)?;
    let sidx = &blob[box_range];
    let version = sidx[8];
    let reference_count_offset = match version {
        1 => 12 + 8 + 16 + 2,
        _ => 12 + 8 + 8 + 2,
    };

    let reference_count = u16::from_be_bytes(
        sidx[reference_count_offset..(reference_count_offset + 2)]
            .try_into()
            .unwrap(),
    );

    if reference_count == 0 {
        return Err(ApiError::new(500, "No segments in SIDX box"));
    }

    let reference_offset = reference_count_offset + 2;
    let segment_reference = &sidx[reference_offset..reference_offset + REFERENCE_SIZE];

    Ok(u32::from_be_bytes(
        segment_reference[DURATION_OFFSET..DURATION_OFFSET + 4]
            .try_into()
            .unwrap(),
    ))
}
