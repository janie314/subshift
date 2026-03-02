# Requirements Checklist: Subtitle Shifter CLI (subshift)

## Core Functionality (P1)
- [ ] **Shift Forward**: Adjust both "start" and "end" times by a positive offset (FR-005).
- [ ] **Shift Backward**: Adjust both "start" and "end" times by a negative offset (FR-005).
- [ ] **Zero-Capping**: Backward shift must cap timestamps at exactly `00:00:00,000` (FR-006).
- [ ] **Clipped Entry Removal**: Remove entries that result in a zero or negative duration (FR-009).

## Inputs & Formats (P1)
- [ ] **File Support**: Accept SRT and VTT input files (FR-001).
- [ ] **Offset Formats**: Support seconds (`1.5`) and milliseconds (`500ms`) (FR-002, FR-003).
- [ ] **Timestamp Formats**: Parse standard SRT (`00:00:20,000`) and VTT (`00:00:20.000`) (FR-004).

## Output & Options (P2)
- [ ] **Output Destination**: Allow specifying a destination file path (FR-008).
- [ ] **Overwrite Support**: Option to overwrite the input file (Story 3).
- [ ] **Format Consistency**: Output modified subtitles in the same format as the input (FR-007).

## Feedback & Quality (P2)
- [ ] **Clipped Notifications**: Print a warning if any entries were removed, including first/last entry details (FR-010).
- [ ] **Content Preservation**: Ensure 100% preservation of subtitle text and formatting (SC-004).
- [ ] **Accuracy**: Shifted timestamps are accurate within 1ms (SC-002).
- [ ] **Performance**: Process 90-minute movie subtitle files in less than 500ms (SC-001).
