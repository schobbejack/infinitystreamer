macro_rules! VIDEO_INIT_FILE {
    () => {
        "../assets/media/video/60fps/init-stream0.m4s"
    };
}
macro_rules! AUDIO_INIT_FILE {
    () => {
        "../assets/media/audio/60fps/init-stream0.m4s"
    };
}
macro_rules! VIDEO_FRAGMENT_FILE {
    () => {
        "../assets/media/video/60fps/chunk-stream0-00001.m4s"
    };
}
macro_rules! AUDIO_FRAGMENT_FILE {
    () => {
        "../assets/media/audio/60fps/chunk-stream0-00001.m4s"
    };
}

pub const VIDEO_INIT_LENGTH: usize = include_bytes!(VIDEO_INIT_FILE!()).len();
pub const AUDIO_INIT_LENGTH: usize = include_bytes!(AUDIO_INIT_FILE!()).len();
pub const VIDEO_FRAGMENT_LENGTH: usize = include_bytes!(VIDEO_FRAGMENT_FILE!()).len();
pub const AUDIO_FRAGMENT_LENGTH: usize = include_bytes!(AUDIO_FRAGMENT_FILE!()).len();

pub const VIDEO_INIT_BLOB: &'static [u8; VIDEO_INIT_LENGTH] = include_bytes!(VIDEO_INIT_FILE!());
pub const AUDIO_INIT_BLOB: &'static [u8; AUDIO_INIT_LENGTH] = include_bytes!(AUDIO_INIT_FILE!());
pub const VIDEO_FRAGMENT_BLOB: &'static [u8; VIDEO_FRAGMENT_LENGTH] =
    include_bytes!(VIDEO_FRAGMENT_FILE!());
pub const AUDIO_FRAGMENT_BLOB: &'static [u8; AUDIO_FRAGMENT_LENGTH] =
    include_bytes!(AUDIO_FRAGMENT_FILE!());
