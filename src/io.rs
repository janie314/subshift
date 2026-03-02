use crate::models::{SubtitleData, SubtitleFormat};
use anyhow::{anyhow, Result};
use rsubs_lib::{SRT, VTT};
use std::fs;
use std::path::Path;

pub fn detect_format(path: &Path) -> Result<SubtitleFormat> {
    let extension = path
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_lowercase());

    match extension.as_deref() {
        Some("srt") => Ok(SubtitleFormat::Srt),
        Some("vtt") => Ok(SubtitleFormat::Vtt),
        _ => Err(anyhow!(
            "Unsupported subtitle format. Only .srt and .vtt are supported."
        )),
    }
}

pub fn parse_subtitles(path: &Path, format: SubtitleFormat) -> Result<SubtitleData> {
    let content = fs::read_to_string(path).map_err(|e| anyhow!("Failed to read file: {:?}", e))?;

    match format {
        SubtitleFormat::Srt => {
            let srt_file =
                SRT::parse(&content).map_err(|e| anyhow!("Failed to parse SRT: {:?}", e))?;
            Ok(SubtitleData::Srt(srt_file))
        }
        SubtitleFormat::Vtt => {
            let vtt_file =
                VTT::parse(&content).map_err(|e| anyhow!("Failed to parse VTT: {:?}", e))?;
            Ok(SubtitleData::Vtt(vtt_file))
        }
    }
}
