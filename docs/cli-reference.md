# CLI Reference

Complete reference for all `veac` commands.

## veac build

Compiles a `.veac` file and renders the output video using FFmpeg.

```
veac build <file> [-o, --output <path>]
```

| Option | Description | Default |
|---|---|---|
| `<file>` | Path to the `.veac` source file | (required) |
| `-o, --output <path>` | Output file path | `output.mp4` |

**Example:**

```bash
veac build main.veac -o my-video.mp4
```

---

## veac check

Validates syntax and semantics without rendering.

```
veac check <file>
```

| Option | Description |
|---|---|
| `<file>` | Path to the `.veac` source file |

Reports errors with line numbers and suggestions. This command is fast and does not invoke FFmpeg, making it suitable for CI/CD pipelines and editor integrations.

**Example:**

```bash
veac check main.veac
```

---

## veac plan

Dry-run mode that shows the FFmpeg commands that would be executed.

```
veac plan <file>
```

| Option | Description |
|---|---|
| `<file>` | Path to the `.veac` source file |

Useful for debugging and understanding the compilation output. No video is rendered.

**Example:**

```bash
veac plan main.veac
```

---

## veac fmt

Formats a `.veac` source file with consistent indentation and spacing.

```
veac fmt <file>
```

| Option | Description |
|---|---|
| `<file>` | Path to the `.veac` source file |

Rewrites the file in place with canonical formatting.

**Example:**

```bash
veac fmt main.veac
```

---

## veac probe

Probes a media file and displays its metadata.

```
veac probe <media-file>
```

| Option | Description |
|---|---|
| `<media-file>` | Path to a media file (video, audio, or image) |

Displays resolution, duration, codec, frame rate, audio channels, and other technical details. Uses FFmpeg's `ffprobe` under the hood.

**Example:**

```bash
veac probe assets/intro.mp4
```

Sample output:

```
File: assets/intro.mp4
Duration: 00:01:30.00
Video: h264, 1920x1080, 30 fps
Audio: aac, 48000 Hz, stereo
```

---

## veac batch

Batch rendering from a template `.veac` file with variable overrides.

```
veac batch <template> --params <csv-file> [-o, --output <dir>]
```

| Option | Description | Default |
|---|---|---|
| `<template>` | Path to the template `.veac` file | (required) |
| `--params <csv-file>` | CSV file with variable overrides per row | (required) |
| `-o, --output <dir>` | Output directory for rendered videos | (required) |

Each row in the CSV file generates a separate video. Column headers correspond to variable names defined in the template.

**Example:**

```bash
veac batch template.veac --params data.csv -o output/
```

Given a `data.csv` like:

```csv
name,subtitle
"Episode 1","Welcome to the show"
"Episode 2","Deep dive into VEAC"
```

This produces `output/1.mp4`, `output/2.mp4`, etc., each with the corresponding variable values substituted into the template.
