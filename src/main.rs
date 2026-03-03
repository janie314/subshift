use anyhow::Result;
use clap::{CommandFactory, Parser};
use std::fs;
use std::path::PathBuf;
use subshift::io::{detect_format, parse_subtitles};
use subshift::models::SubtitleData;
use subshift::parse_offset;
use subshift::shifter::shift_subtitles;

#[derive(Parser, Debug)]
#[command(name = "subshift")]
#[command(about = "Shift subtitle file timestamps forwards or backwards.", long_about = None)]
struct Cli {
    /// Path to the subtitle file (.srt or .vtt)
    #[arg(required = false)]
    input_file: Option<PathBuf>,

    /// Time offset to shift (e.g., +1.5s, -500ms, or 2.0 for seconds)
    #[arg(required = false)]
    offset: Option<String>,

    /// Specify a different output file
    #[arg(short = 'o', long)]
    output: Option<PathBuf>,

    /// Overwrite the input file
    #[arg(short = 'w', long)]
    overwrite: bool,
}

#[cfg(test)]
mod cli_tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn parse_with_overwrite_and_offset() {
        let args = ["subshift", "-w", "example.srt", "+0"];
        let cli = Cli::parse_from(args.clone());
        // log parse results
        let log = format!(
            "parsed overwrite={} input={:?} offset={:?}\n",
            cli.overwrite, cli.input_file, cli.offset
        );
        let _ = std::fs::write("/tmp/cli_parse_result.log", log);
        assert!(cli.overwrite);
        assert_eq!(cli.input_file.unwrap(), PathBuf::from("example.srt"));
        assert_eq!(cli.offset.unwrap(), "+0");
    }

    #[test]
    fn help_prints_when_missing() {
        let result = Cli::try_parse_from(["subshift"]);
        assert!(result.is_ok()); // parse succeeds and returns struct with None fields
        let cli = result.unwrap();
        assert!(cli.input_file.is_none());
        assert!(cli.offset.is_none());
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // if required arguments missing, print help and exit success
    if cli.input_file.is_none() || cli.offset.is_none() {
        let mut cmd = Cli::command();
        cmd.print_help()?;
        println!();
        return Ok(());
    }

    let input_file = cli.input_file.as_ref().unwrap();
    let offset_ms = parse_offset(cli.offset.as_ref().unwrap())?;
    let format = detect_format(input_file)?;
    let sub_data = parse_subtitles(input_file, format)?;

    let (shifted_data, result) = shift_subtitles(sub_data, offset_ms);
    let mut logbuf = String::new();

    let output_path = if cli.overwrite {
        input_file.clone()
    } else if let Some(output) = cli.output.clone() {
        output
    } else {
        let mut new_path = input_file.clone();
        let stem = new_path.file_stem().unwrap().to_str().unwrap();
        let extension = new_path.extension().unwrap().to_str().unwrap();
        new_path.set_file_name(format!("{}_shifted.{}", stem, extension));
        new_path
    };

    let out_str = match shifted_data {
        SubtitleData::Srt(srt_file) => srt_file.to_string(),
        SubtitleData::Vtt(vtt_file) => vtt_file.to_string(),
    };
    logbuf.push_str(&format!("writing output to {:?}\n", output_path));
    fs::write(&output_path, out_str)?;

    println!("Successfully shifted subtitles to {:?}", output_path);
    logbuf.push_str("printed success\n");

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
        logbuf.push_str(&format!("removed count {}\n", result.clipped_count));
    }

    let _ = std::fs::write("/tmp/subshift.log", logbuf);
    Ok(())
}
