use crate::{
    api_error::ApiError,
    mp4utils::{get_fragment_duration, get_timescale},
};

pub struct FragmentParams<const N: usize, const M: usize> {
    pub init: &'static [u8; N],
    pub fragment: &'static [u8; M],
    pub timescale: u32,
    pub duration: u32,
    pub extinf: f32,
}

impl<const N: usize, const M: usize> FragmentParams<N, M> {
    pub fn new(init: &'static [u8; N], fragment: &'static [u8; M]) -> Result<Self, ApiError> {
        let timescale = get_timescale(init)?;
        let duration = get_fragment_duration(fragment)?;
        let extinf = duration as f32 / timescale as f32;
        Ok(FragmentParams::<N, M> {
            init,
            fragment,
            timescale: timescale,
            duration: duration,
            extinf: extinf,
        })
    }
}
