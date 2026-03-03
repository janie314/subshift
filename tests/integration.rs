use std::fs;
use std::path::Path;
use std::process::Command;

#[test]
fn test_cli_help() {
    let status = Command::new("cargo")
        .args(["run", "--", "--help"])
        .status()
        .expect("Failed to execute cargo run");

    assert!(status.success());
}

#[test]
fn test_shift_forward_srt() {
    let output_file = "tests/fixtures/test_shifted_forward.srt";
    if Path::new(output_file).exists() {
        fs::remove_file(output_file).unwrap();
    }

    let status = Command::new("cargo")
        .args([
            "run",
            "--",
            "tests/fixtures/test.srt",
            "+1.5s",
            "-o",
            output_file,
        ])
        .status()
        .expect("Failed to execute subshift");

    assert!(status.success());
    let content = fs::read_to_string(output_file).unwrap();
    assert!(content.contains("00:00:02,500 --> 00:00:05,500"));
}

#[test]
fn test_shift_backward_srt_clipping() {
    let output_file = "tests/fixtures/test_shifted_backward.srt";
    if Path::new(output_file).exists() {
        fs::remove_file(output_file).unwrap();
    }

    // Shift back by 2 seconds. The first entry (1s to 4s) should become (0s to 2s).
    // If we shift back by 5 seconds, it should be removed.
    let status = Command::new("cargo")
        .args([
            "run",
            "--",
            "tests/fixtures/test.srt",
            "-2s",
            "-o",
            output_file,
        ])
        .status()
        .expect("Failed to execute subshift");

    assert!(status.success());
    let content = fs::read_to_string(output_file).unwrap();
    assert!(content.contains("00:00:00,000 --> 00:00:02,000"));
    assert!(content.contains("00:00:03,000 --> 00:00:06,000"));
}

#[test]
fn test_shift_vtt() {
    let output_file = "tests/fixtures/test_shifted.vtt";
    if Path::new(output_file).exists() {
        fs::remove_file(output_file).unwrap();
    }

    let status = Command::new("cargo")
        .args([
            "run",
            "--",
            "tests/fixtures/test.vtt",
            "1.0",
            "-o",
            output_file,
        ])
        .status()
        .expect("Failed to execute subshift");

    assert!(status.success());
    let content = fs::read_to_string(output_file).unwrap();
    assert!(content.contains("00:00:02.000 --> 00:00:05.000"));
}
