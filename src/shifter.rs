use crate::models::SubtitleData;
use std::ops::{Add, Sub};
use std::time::Duration as StdDuration;

#[derive(Debug)]
pub struct ShiftResult {
    pub clipped_count: usize,
    pub first_removed: Option<String>,
    pub last_removed: Option<String>,
}

pub fn shift_subtitles(sub_data: SubtitleData, offset_ms: i64) -> (SubtitleData, ShiftResult) {
    let mut clipped_count = 0;
    let mut first_removed = None;
    let mut last_removed = None;
    let abs_offset = StdDuration::from_millis(offset_ms.abs() as u64);

    match sub_data {
        SubtitleData::Srt(mut srt_file) => {
            srt_file.lines.retain_mut(|line| {
                if offset_ms >= 0 {
                    line.start = line.start.add(abs_offset);
                    line.end = line.end.add(abs_offset);
                } else {
                    let end_ms = (line.end - time::Time::MIDNIGHT).whole_milliseconds();
                    let start_ms = (line.start - time::Time::MIDNIGHT).whole_milliseconds();

                    if end_ms + (offset_ms as i128) <= 0 {
                        clipped_count += 1;
                        if first_removed.is_none() {
                            first_removed = Some(line.text.clone());
                        }
                        last_removed = Some(line.text.clone());
                        return false;
                    }

                    if start_ms + (offset_ms as i128) <= 0 {
                        let shift_to_zero = StdDuration::from_millis(start_ms as u64);
                        line.start = line.start.sub(shift_to_zero);
                    } else {
                        line.start = line.start.sub(abs_offset);
                    }
                    line.end = line.end.sub(abs_offset);
                }
                true
            });
            (
                SubtitleData::Srt(srt_file),
                ShiftResult {
                    clipped_count,
                    first_removed,
                    last_removed,
                },
            )
        }
        SubtitleData::Vtt(mut vtt_file) => {
            vtt_file.lines.retain_mut(|line| {
                if offset_ms >= 0 {
                    line.start = line.start.add(abs_offset);
                    line.end = line.end.add(abs_offset);
                } else {
                    let end_ms = (line.end - time::Time::MIDNIGHT).whole_milliseconds();
                    let start_ms = (line.start - time::Time::MIDNIGHT).whole_milliseconds();

                    if end_ms + (offset_ms as i128) <= 0 {
                        clipped_count += 1;
                        if first_removed.is_none() {
                            first_removed = Some(line.text.clone());
                        }
                        last_removed = Some(line.text.clone());
                        return false;
                    }

                    if start_ms + (offset_ms as i128) <= 0 {
                        let shift_to_zero = StdDuration::from_millis(start_ms as u64);
                        line.start = line.start.sub(shift_to_zero);
                    } else {
                        line.start = line.start.sub(abs_offset);
                    }
                    line.end = line.end.sub(abs_offset);
                }
                true
            });
            (
                SubtitleData::Vtt(vtt_file),
                ShiftResult {
                    clipped_count,
                    first_removed,
                    last_removed,
                },
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rsubs_lib::{SRTLine, VTTLine, SRT, VTT};
    use time::Time;

    fn make_srt() -> SubtitleData {
        let mut srt = SRT {
            lines: vec![SRTLine {
                sequence_number: 1,
                start: Time::MIDNIGHT,
                end: Time::MIDNIGHT,
                text: "hello".into(),
            }],
        };
        SubtitleData::Srt(srt)
    }

    #[test]
    fn zero_offset_leaves_data() {
        let data = make_srt();
        let (shifted, result) = shift_subtitles(data, 0);
        if let SubtitleData::Srt(srt) = shifted {
            assert_eq!(srt.lines.len(), 1);
        } else {
            panic!("expected srt");
        }
        assert_eq!(result.clipped_count, 0);
    }
}
