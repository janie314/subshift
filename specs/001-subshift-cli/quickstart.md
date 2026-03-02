# Quickstart: Subtitle Shifter CLI (subshift)

## Prerequisites
- Rust (2024 Edition) installed.

## Installation

```bash
cargo build --release
# Binary will be at target/release/subshift
```

## Basic Usage

### Shift subtitles forward (delayed audio)

```bash
# Shift forward by 1.5 seconds
subshift movie.srt +1.5s -o movie_delayed.srt
```

### Shift subtitles backward (early audio)

```bash
# Shift backward by 500 milliseconds and overwrite
subshift movie.srt -500ms --overwrite
```

### Output to a different file

```bash
subshift movie.vtt 2 -o shifted.vtt
```

## Verification

After shifting, you can verify the results by opening the file in a text editor or a media player like VLC.
All timestamps in the output file should be shifted by the specified amount.
Any subtitle entry that would have a negative timestamp is capped at 00:00:00 or removed if the entire duration becomes negative.
