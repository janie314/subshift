# Research: Subtitle Shifter CLI (subshift)

## Decision: Subtitle Parsing Library
**Chosen**: `rsubs-lib`
**Rationale**: It is a comprehensive library that supports both SRT and WebVTT, and it already has built-in support for modifying timestamps (shifting time). This aligns perfectly with our requirement to shift subtitles while preserving other content.
**Alternatives considered**: 
- `subtp`: Modern and lightweight, but `rsubs-lib` seems more feature-rich for manipulation.
- `aspasia`: Good for messy files, but `rsubs-lib` is more standard for our needs.
- Custom parser: Rejected due to the complexity of handling both SRT and WebVTT correctly, including multi-line text and styling.

## Decision: Time/Duration Parsing
**Chosen**: `humantime` + custom logic for negative offsets.
**Rationale**: `humantime` is the industry standard for parsing durations like `1.5s` and `500ms`. Since `humantime` doesn't natively handle negative signs for durations, we will pre-parse the sign and use the absolute duration for shifting.
**Alternatives considered**: 
- `duration-str`: Similar to `humantime`.
- Custom regex: Possible, but `humantime` is more robust.

## Decision: CLI Argument Parsing
**Chosen**: `clap` (v4)
**Rationale**: Standard Rust CLI library with great support for derive-based argument parsing, subcommands (if needed), and validation.
**Alternatives considered**: `structopt` (deprecated in favor of `clap` v4).

## Decision: Error Handling
**Chosen**: `anyhow`
**Rationale**: Best for CLI applications where we want to provide clear error messages and backtraces without defining complex custom error types for every failure.

## Decision: File Encoding
**Chosen**: UTF-8 (Strict)
**Rationale**: SRT and WebVTT are typically UTF-8. We will assume UTF-8 for now, as is standard for modern Rust tools.
**Alternatives considered**: `encoding_rs` for legacy SRTs, but this is deferred to a future "v2" requirement if needed.
