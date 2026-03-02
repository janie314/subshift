# Data Model: Subtitle Shifter CLI (subshift)

## Entities

### SubtitleEntry
Represents a single subtitle block.
- **Start Time**: Milliseconds from start.
- **End Time**: Milliseconds from start.
- **Content**: Original text content (including formatting tags).
- **Index**: Entry number (for SRT).

### SubtitleFile
Represents the entire subtitle file.
- **Format**: `SRT` or `VTT`.
- **Entries**: Ordered collection of `SubtitleEntry`.

### ShiftConfig
Represents the user's request.
- **Input Path**: Path to the source file.
- **Output Path**: Optional path to the destination file.
- **Offset**: Signed duration (milliseconds).
- **Overwrite**: Boolean flag.

## State Transitions

1. **Parse**: `File Path` → `SubtitleFile` (using `rsubs-lib`)
2. **Validate**: Check for file existence, valid subtitle format, and valid offset string.
3. **Shift**: `SubtitleFile` + `Offset` → `Shifted SubtitleFile`
   - For each `SubtitleEntry`:
     - `new_start = max(0, start + offset)`
     - `new_end = max(0, end + offset)`
     - If `new_end <= 0`, remove entry.
4. **Serialize**: `Shifted SubtitleFile` → `String` / `File Buffer`
5. **Output**: Write to `Output Path` or overwrite `Input Path`.
