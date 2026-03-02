use clap::Parser;
use std::path::PathBuf;
use anyhow::Result;
use subshift::io::{detect_format, parse_subtitles};
use std::fs;
use subshift::shifter::shift_subtitles;
use subshift::parse_offset;
use subshift::models::SubtitleData;

#[derive(Parser)]
#[command(name = "subshift")]
#[command(about = "Shift subtitle file timestamps forwards or backwards.", long_about = None)]
struct Cli {
    /// Path to the subtitle file (.srt or .vtt)
    input_file: PathBuf,

    /// Time offset to shift (e.g., +1.5s, -500ms, or 2.0 for seconds)
    offset: String,

    /// Specify a different output file
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Overwrite the input file
    #[arg(short, long)]
    overwrite: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let offset_ms = parse_offset(&cli.offset)?;
    let format = detect_format(&cli.input_file)?;
    let sub_data = parse_subtitles(&cli.input_file, format)?;

    let (shifted_data, result) = shift_subtitles(sub_data, offset_ms);

    let output_path = if cli.overwrite {
        cli.input_file.clone()
    } else if let Some(output) = cli.output {
        output
    } else {
        let mut new_path = cli.input_file.clone();
        let stem = new_path.file_stem().unwrap().to_str().unwrap();
        let extension = new_path.extension().unwrap().to_str().unwrap();
        new_path.set_file_name(format!("{}_shifted.{}", stem, extension));
        new_path
    };

    let out_str = match shifted_data {
        SubtitleData::Srt(srt_file) => srt_file.to_string(),
        SubtitleData::Vtt(vtt_file) => vtt_file.to_string(),
    };
    fs::write(&output_path, out_str)?;

    println!("Successfully shifted subtitles to {:?}", output_path);

    if result.clipped_count > 0 {
        eprintln!(
            "Warning: {} subtitle entries were removed because they were shifted before 00:00:00.",
            result.clipped_count
        );
        if let Some(first) = result.first_removed {
            eprintln!("First removed: {}", first.trim());
        }
        if let Some(last) = result.last_removed {
            eprintln!("Last removed: {}", last.trim());
        }
    }

    Ok(())
}
