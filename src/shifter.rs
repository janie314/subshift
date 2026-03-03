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
    use rsubs_lib::{SRTLine, SRT};
    use time::Time;

    fn make_srt() -> SubtitleData {
        let srt = SRT {
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

    #[test]
    fn positive_offset_moves_lines_forward() {
        // one line starting at 1s ending at 2s
        let srt = SRT {
            lines: vec![SRTLine {
                sequence_number: 1,
                start: Time::MIDNIGHT + time::Duration::seconds(1),
                end: Time::MIDNIGHT + time::Duration::seconds(2),
                text: "a".into(),
            }],
        };
        let (shifted, result) = shift_subtitles(SubtitleData::Srt(srt), 1500);
        if let SubtitleData::Srt(srt) = shifted {
            let line = &srt.lines[0];
            assert_eq!((line.start - Time::MIDNIGHT).whole_milliseconds(), 2500);
            assert_eq!((line.end - Time::MIDNIGHT).whole_milliseconds(), 3500);
        } else {
            panic!("expected srt");
        }
        assert_eq!(result.clipped_count, 0);
    }

    #[test]
    fn negative_offset_partial_clip_and_shift() {
        // two lines: first will hit zero start, second stays positive
        let srt = SRT {
            lines: vec![
                SRTLine {
                    sequence_number: 1,
                    start: Time::MIDNIGHT + time::Duration::seconds(1),
                    end: Time::MIDNIGHT + time::Duration::seconds(4),
                    text: "first".into(),
                },
                SRTLine {
                    sequence_number: 2,
                    start: Time::MIDNIGHT + time::Duration::seconds(5),
                    end: Time::MIDNIGHT + time::Duration::seconds(6),
                    text: "second".into(),
                },
            ],
        };
        let (shifted, result) = shift_subtitles(SubtitleData::Srt(srt), -1500);
        if let SubtitleData::Srt(srt) = shifted {
            assert_eq!(srt.lines.len(), 2);
            // first line start should be clamped to 0, end reduced by 1.5s
            let first = &srt.lines[0];
            assert_eq!((first.start - Time::MIDNIGHT).whole_milliseconds(), 0);
            assert_eq!((first.end - Time::MIDNIGHT).whole_milliseconds(), 2500);
            // second line shifted normally
            let second = &srt.lines[1];
            assert_eq!((second.start - Time::MIDNIGHT).whole_milliseconds(), 3500);
            assert_eq!((second.end - Time::MIDNIGHT).whole_milliseconds(), 4500);
        } else {
            panic!("expected srt");
        }
        assert_eq!(result.clipped_count, 0);
    }

    #[test]
    fn negative_offset_remove_all() {
        // both lines will be removed when offset pushes them entirely before zero
        let srt = SRT {
            lines: vec![
                SRTLine {
                    sequence_number: 1,
                    start: Time::MIDNIGHT + time::Duration::seconds(1),
                    end: Time::MIDNIGHT + time::Duration::seconds(4),
                    text: "first".into(),
                },
                SRTLine {
                    sequence_number: 2,
                    start: Time::MIDNIGHT + time::Duration::seconds(5),
                    end: Time::MIDNIGHT + time::Duration::seconds(6),
                    text: "second".into(),
                },
            ],
        };
        let (shifted, result) = shift_subtitles(SubtitleData::Srt(srt), -6000);
        if let SubtitleData::Srt(srt) = shifted {
            assert!(srt.lines.is_empty());
        } else {
            panic!("expected srt");
        }
        assert_eq!(result.clipped_count, 2);
        assert_eq!(result.first_removed.as_deref(), Some("first"));
        assert_eq!(result.last_removed.as_deref(), Some("second"));
    }

    #[test]
    fn vtt_behaviour_matches_srt() {
        use rsubs_lib::{VTTLine, VTT};
        use std::collections::HashMap;

        let vtt = VTT {
            regions: Vec::new(),
            styles: Vec::new(),
            lines: vec![VTTLine {
                identifier: None,
                start: Time::MIDNIGHT + time::Duration::seconds(2),
                end: Time::MIDNIGHT + time::Duration::seconds(3),
                settings: HashMap::new(),
                text: "vtt".into(),
            }],
        };

        let (shifted, result) = shift_subtitles(SubtitleData::Vtt(vtt), 1000);
        if let SubtitleData::Vtt(vtt2) = shifted {
            let line = &vtt2.lines[0];
            assert_eq!((line.start - Time::MIDNIGHT).whole_milliseconds(), 3000);
            assert_eq!((line.end - Time::MIDNIGHT).whole_milliseconds(), 4000);
        } else {
            panic!("expected vtt");
        }
        assert_eq!(result.clipped_count, 0);
    }
}
