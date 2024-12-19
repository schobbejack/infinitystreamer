use crate::{
    api_error::ApiError,
    mp4utils::{get_dimension, get_fragment_duration, get_timescale},
};

use super::dimension::Dimension;

pub struct FragmentParams<const N: usize, const M: usize> {
    pub init: &'static [u8; N],
    pub fragment: &'static [u8; M],
    pub timescale: u32,
    pub duration: u32,
    pub dimension: Dimension,
    pub sample_count: u32,
    pub extinf: f32,
}

impl<const N: usize, const M: usize> FragmentParams<N, M> {
    pub fn new(init: &'static [u8; N], fragment: &'static [u8; M]) -> Result<Self, ApiError> {
        let timescale = get_timescale(init)?;
        let (duration, sample_count) = get_fragment_duration(fragment)?;
        let extinf = duration as f32 / timescale as f32;
        let dimension = get_dimension(init)?;
        Ok(FragmentParams::<N, M> {
            init,
            fragment,
            timescale: timescale,
            duration: duration,
            dimension: dimension,
            sample_count: sample_count,
            extinf: extinf,
        })
    }

    pub fn bitrate(&self) -> u32 {
        (self.fragment.len() as u64 * 8 * self.timescale as u64 / self.duration as u64) as u32
    }
}
