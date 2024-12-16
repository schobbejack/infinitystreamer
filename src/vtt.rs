use crate::{api_error::ApiError, models::fragment::FragmentParams};

const VTT_TIMESCALE: i64 = 90000;

pub fn generate<const N: usize, const M: usize>(
    sequence: i64,
    fragment: &FragmentParams<N, M>,
) -> Result<String, ApiError> {
    // get current date/time from sequence, assume vtt duration == video duration

    let fragment_start =
        sequence * fragment.duration as i64 * VTT_TIMESCALE / fragment.timescale as i64;
    let fragment_end =
        fragment_start + fragment.duration as i64 * VTT_TIMESCALE / fragment.timescale as i64;
    let vttheader = format!(include_str!("../assets/media/template.vtt"), fragment_start);

    let cues = create_cue_list(fragment_start, fragment_end)?;

    Ok(vttheader + &cues)
}

fn format_cue_time(time_offset_ms: i64) -> String {
    let hh = time_offset_ms / 3600000;
    let mm = (time_offset_ms / 60000) % 60;
    let ss = (time_offset_ms / 1000) % 60;
    let ms = time_offset_ms - (time_offset_ms / 1000) * 1000;

    format!("{:0>2}:{:0>2}:{:0>2}.{:0>3}", hh, mm, ss, ms)
}

fn create_cue_list(fragment_start: i64, fragment_end: i64) -> Result<String, ApiError> {
    const LINES: &[&str] = &[
        "&#x1F60B; &#x1f918;",
        "&#x1F61B; &#x1f918;",
        "&#x1F61C; &#x1f918;",
        "&#x1F92A; &#x1f918;",
        "Woooosshh...",
    ];

    let mut cues = String::new();

    let vtt_start = (fragment_start + VTT_TIMESCALE - 1) / VTT_TIMESCALE * VTT_TIMESCALE;
    let offset = vtt_start - fragment_start;
    let start_line = (vtt_start / VTT_TIMESCALE) as usize;
    let line_count = ((fragment_end + VTT_TIMESCALE - 1) / VTT_TIMESCALE) as usize - start_line;
    for line in 0..line_count {
        let cue_start = offset * 1000 / VTT_TIMESCALE + line as i64 * 1000;
        let cue_end = offset * 1000 / VTT_TIMESCALE + (line as i64 + 1) * 1000;

        cues += &format!(
            "\n{} --> {}\n{}\n",
            format_cue_time(cue_start),
            format_cue_time(cue_end),
            LINES[(start_line + line) % LINES.len()]
        );
    }

    Ok(cues)
}
