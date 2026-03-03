use rsubs_lib::{SRT, VTT};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy)]
pub enum SubtitleFormat {
    Srt,
    Vtt,
}

pub enum SubtitleData {
    Srt(SRT),
    Vtt(VTT),
}

pub struct ShiftConfig {
    pub input_file: PathBuf,
    pub output_file: PathBuf,
    pub offset_ms: i64,
}
