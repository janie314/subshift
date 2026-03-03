# subshift

Shift subtitle file timestamps forwards or backwards.

# Usage

```
Usage: subshift [OPTIONS] [INPUT_FILE] [OFFSET]

Arguments:
  [INPUT_FILE]  Path to the subtitle file (.srt or .vtt)
  [OFFSET]      Time offset to shift (e.g., +1.5s, -500ms, or 2.0 for seconds)

Options:
  -o, --output <OUTPUT>  Specify a different output file
  -w, --overwrite        Overwrite the input file
  -h, --help             Print help
```

## Installation

```sh
make install
```

## Logging

The CLI now uses [`tracing`](https://docs.rs/tracing) for output. By default
messages at `info` level and above are shown; use the `RUST_LOG` environment
variable to adjust the filter, e.g.:

```sh
RUST_LOG=warn subshift example.srt +1.0
```

Diagnostic and warning messages are printed to stderr, so they don't interfere
with piping the shifted subtitles to other programs.
