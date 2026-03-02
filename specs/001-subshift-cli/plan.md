# Implementation Plan: [FEATURE]

**Branch**: `[###-feature-name]` | **Date**: [DATE] | **Spec**: [link]
**Input**: Feature specification from `/specs/[###-feature-name]/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/plan-template.md` for the execution workflow.

## Summary

A Rust-based CLI tool to shift subtitle (SRT, VTT) timestamps forwards or backwards. It supports seconds and milliseconds offsets, handles edge cases like negative timestamps and clipped subtitles, and allows overwriting or saving to a new file. The tool focuses on performance (<500ms for a 90-minute movie) and 100% data preservation.

## Technical Context

**Language/Version**: Rust (2024 Edition)  
**Primary Dependencies**: 
- `clap` (v4)
- `rsubs-lib` (Subtitle parsing and manipulation)
- `humantime` (Duration parsing)
- `anyhow` (Error handling)
**Storage**: N/A (direct file I/O)  
**Testing**: `cargo test` (unit and integration tests)  
**Target Platform**: Linux (and other Tier 1 Rust platforms)  
**Project Type**: CLI tool  
**Performance Goals**: Process 2000 entries in < 500ms (SC-001)  
**Constraints**: 100% preservation of non-timestamp content (SC-004), 1ms accuracy (SC-002)  
**Scale/Scope**: Single binary, handling files with thousands of entries.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Requirement Verification**: All requirements from `spec.md` are covered.
- **Library Choice**: `rsubs-lib` provides the most comprehensive feature set for both SRT and VTT.
- **Simplicity**: Single CLI tool with minimal dependencies, adhering to the project's goal of performance and data integrity.
- **TDD**: Tests are planned for both unit logic and end-to-end CLI scenarios.


## Project Structure

### Documentation (this feature)

```text
specs/001-subshift-cli/
├── plan.md              # This file
├── research.md          # Subtitle and time parsing research
├── data-model.md        # Entities: SubtitleFile, SubtitleEntry, ShiftConfig
├── quickstart.md        # Installation and usage examples
├── contracts/
│   └── cli-contract.md  # CLI argument and flag definitions
└── checklists/
    └── requirements.md  # Original requirements checklist
```

### Source Code (repository root)

```text
src/
├── main.rs         # CLI entry point, argument parsing (clap)
├── lib.rs          # Public interface for shifting logic
├── shifter.rs      # Core shifting service (manipulates SubtitleFile)
├── io.rs           # File I/O helpers (SRT/VTT detection)
└── models.rs       # Internal data structures

tests/
├── fixtures/       # Sample .srt and .vtt files for testing
└── integration.rs  # End-to-end tests for CLI commands
```

**Structure Decision**: A standard single-project Rust structure is chosen. Shifting logic will be separated from CLI logic to allow for easy unit testing. I/O will handle format detection and delegates to `rsubs-lib`.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| N/A | | |

