# Tasks: Subtitle Shifter CLI (subshift)

## Phase 1: Project Setup & CLI Scaffolding
- [X] **Task 1.1**: Initialize `Cargo.toml` with dependencies (`clap`, `rsubs-lib`, `humantime`, `anyhow`).
- [X] **Task 1.2**: Implement basic CLI argument parsing in `src/main.rs` using `clap` (Input, Offset, Output, Overwrite).
- [X] **Task 1.3**: Add a simple "Hello, world" integration test to verify the CLI binary builds and runs.

## Phase 2: Core Shifting Logic
- [X] **Task 2.1**: Implement subtitle format detection and parsing in `src/io.rs` using `rsubs-lib`.
- [X] **Task 2.2**: Implement the `ShiftConfig` and signed duration parsing logic (handling `+` and `-` signs).
- [X] **Task 2.3**: Implement the core shifting function in `src/shifter.rs`.
    - [X] Handle forward shift.
    - [X] Handle backward shift with `00:00:00` capping.
    - [X] Handle entry removal for negative/zero durations.
- [X] **Task 2.4**: Unit tests for shifting logic (SRT and VTT formats).

## Phase 3: File I/O & User Feedback
- [X] **Task 3.1**: Implement file writing logic (handling new output path vs. overwrite).
- [X] **Task 3.2**: Implement clipped entry tracking and warning notifications (SC-002, SC-004).
- [X] **Task 3.3**: Verify data preservation (non-timestamp content) via unit tests.

## Phase 4: Final Validation
- [ ] **Task 4.1**: Integration tests for "Shift Forward" (Story 1).
- [ ] **Task 4.2**: Integration tests for "Shift Backward" (Story 2).
- [ ] **Task 4.3**: Performance benchmark to verify <500ms for ~2000 entries (SC-001).
- [ ] **Task 4.4**: Final documentation review and cleanup.
