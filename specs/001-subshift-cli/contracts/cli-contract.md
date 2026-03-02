# CLI Contract: Subtitle Shifter CLI (subshift)

## Command Usage

```bash
subshift <INPUT_FILE> <OFFSET> [OPTIONS]
```

## Arguments

| Argument | Description | Example |
|---|---|---|
| `INPUT_FILE` | Path to the subtitle file (.srt or .vtt). | `movie.srt` |
| `OFFSET` | Time shift with sign and unit (s or ms). | `+1.5s`, `-500ms`, `2.0` (default s) |

## Options

| Option | Long | Short | Description |
|---|---|---|---|
| `-o` | `--output` | `FILE` | Specify a different output file. |
| `-w` | `--overwrite` | | Overwrite the input file. |
| `-h` | `--help` | | Show help information. |
| `-V` | `--version` | | Show version information. |

## Success/Failure Behaviors

- **Success (0)**: Prints "Successfully shifted subtitles to [PATH]".
- **Warning (0)**: If entries were clipped, prints: "Warning: [N] subtitle entries were removed because they were shifted before 00:00:00. First removed: [X], Last removed: [Y]".
- **Error (1)**:
  - File not found.
  - Invalid subtitle format.
  - Invalid offset format.
  - Permission denied.
