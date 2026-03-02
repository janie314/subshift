# Feature Specification: Subtitle Shifter CLI (subshift)

**Feature Branch**: `001-subshift-cli`  
**Created**: 2026-03-02  
**Status**: Draft  
**Input**: User description: "Make a CLI called subshift that shifts subtitle files' timestamps forwards or backwards in time."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Shift Subtitle Timing Forward (Priority: P1)

As a viewer watching a movie where the subtitles appear too early, I want to shift all subtitle timestamps forward by a specific duration so that they sync perfectly with the audio.

**Why this priority**: This is the core functionality of the tool and the primary reason users would seek it out.

**Independent Test**: Can be tested by providing an SRT file and a positive offset (e.g., 2 seconds), then verifying that every timestamp in the output file is exactly 2 seconds later than the input.

**Acceptance Scenarios**:

1. **Given** a valid SRT file and an offset of "+1.5s", **When** I run `subshift`, **Then** the output file contains the same subtitle text but with all start and end times increased by 1.5 seconds.
2. **Given** a valid SRT file and an offset of "500ms", **When** I run `subshift`, **Then** the output file contains the same subtitle text but with all start and end times increased by 0.5 seconds.

---

### User Story 2 - Shift Subtitle Timing Backward (Priority: P1)

As a viewer watching a movie where the subtitles appear too late, I want to shift all subtitle timestamps backward by a specific duration so that they sync perfectly with the audio.

**Why this priority**: Equally important as shifting forward; covers the other half of the "desync" problem.

**Independent Test**: Can be tested by providing an SRT file and a negative offset (e.g., -1 second), then verifying that every timestamp is exactly 1 second earlier.

**Acceptance Scenarios**:

1. **Given** a valid SRT file and an offset of "-2s", **When** I run `subshift`, **Then** all timestamps are decreased by 2 seconds.
2. **Given** a timestamp that would become negative (e.g., 00:00:01,000 shifted by -2s), **When** I run `subshift`, **Then** the timestamp is capped at 00:00:00,000.

---

### User Story 3 - Save to a New File or Overwrite (Priority: P2)

As a user, I want to choose whether to create a new shifted subtitle file or overwrite the existing one so that I can preserve my original files if needed.

**Why this priority**: Important for user workflow and data safety, but the core shifting logic (P1) is the prerequisite.

**Independent Test**: Can be tested by checking if the original file remains unchanged when a new output path is specified, or if the original file's content is updated when the "overwrite" option is used.

**Acceptance Scenarios**:

1. **Given** an input file `movie.srt`, **When** I specify an output file `movie_shifted.srt`, **Then** `movie.srt` remains unchanged and `movie_shifted.srt` is created with shifted timestamps.
2. **Given** an input file `movie.srt`, **When** I use the overwrite flag, **Then** `movie.srt` is updated with the shifted timestamps.

---

### Edge Cases

- **Negative Resulting Timestamps**: If a backward shift results in a time before 00:00:00, the time must be set to exactly 00:00:00.
- **Clipped Subtitles**: If shifting causes a subtitle's end time to be capped at 00:00:00, the entry MUST be removed from the output.
- **User Notification**: The system MUST print a warning if subtitles were removed, indicating the first and last subtitles that were clipped.
- **Invalid File Formats**: How the tool handles non-subtitle files or corrupted subtitle files.
- **Large Files**: Handling files with thousands of entries efficiently.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST accept a subtitle file (SRT or VTT) as input.
- **FR-002**: System MUST accept a time offset as a command-line argument.
- **FR-003**: System MUST support time offsets in seconds (e.g., `1.5`, `-2.0`) and milliseconds (e.g., `500ms`).
- **FR-004**: System MUST parse standard subtitle timestamp formats (e.g., `00:00:20,000` for SRT, `00:00:20.000` for VTT).
- **FR-005**: System MUST adjust both "start" and "end" times for every subtitle entry in the file.
- **FR-006**: System MUST cap adjusted timestamps at a minimum of `00:00:00,000`.
- **FR-007**: System MUST output the modified subtitles in the same format (SRT or VTT) as the input file.
- **FR-008**: System MUST allow the user to specify a destination file path.
- **FR-009**: System MUST remove subtitle entries that result in a zero or negative duration after shifting.
- **FR-010**: System MUST notify the user via a warning if any subtitles were removed, listing the first and last removed entries.

### Key Entities

- **Subtitle Entry**: Represents a single block of text with a start time, end time, and the text content.
- **Timestamp**: Represents a specific point in time (Hours, Minutes, Seconds, Milliseconds).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can shift a standard 90-minute movie subtitle file (approx. 1000-2000 entries) in less than 500ms.
- **SC-002**: Shifted timestamps are accurate to within 1 millisecond of the requested offset.
- SC-003: Modified files are successfully parsed by standard media players without errors.
- **SC-004**: 100% of subtitle text and formatting (bold, italic, etc.) is preserved during the shifting process.
