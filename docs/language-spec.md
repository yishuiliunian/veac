# VEAC Language Specification

## Overview

VEAC (Video Editing as Code) is a declarative, non-Turing-complete domain-specific language for video editing. It compiles `.veac` source files into FFmpeg commands to produce video output.

## File Structure

```
project-name/
├── main.veac          # Main source file
├── assets/            # Media resources
│   ├── intro.mp4
│   ├── bgm.mp3
│   └── logo.png
└── veac.toml          # Project config (optional, future)
```

## Syntax

### Project Declaration

```veac
project "name" {
    resolution = "1920x1080"   // WIDTHxHEIGHT
    fps        = 30            // Frames per second
    format     = "mp4"         // mp4, mkv, webm, mov
    codec      = "h264"        // h264, h265, vp9, av1
    quality    = "high"        // low, medium, high, lossless
}
```

### Asset Declaration

```veac
asset name = video("./path/to/file.mp4")
asset name = audio("./path/to/file.mp3")
asset name = image("./path/to/file.png")
```

### Variables

```veac
let fade_duration = 1s
let default_volume = 0.8
```

### Time Literals

| Format | Example | Description |
|--------|---------|-------------|
| Seconds | `3.5s` | Float seconds |
| Milliseconds | `500ms` | Milliseconds |
| Frames | `84f` | Frame number (converted via fps) |
| SMPTE | `"00:01:30:12"` | HH:MM:SS:FF timecode |

### Timeline

```veac
timeline main {
    track video {
        clip asset_ref {
            from     = 0s
            to       = 10s
            volume   = 0.5     // 0.0 - 1.0
        }
    }

    track audio {
        clip bgm {
            volume = 0.8
        }
    }

    track text {
        text "Hello World" {
            at       = 3s
            duration = 5s
            font     = "Arial"
            size     = 48
            color    = #FFFFFF
            position = "center"
        }
    }
}
```

### Position Values

`center`, `top`, `bottom`, `left`, `right`, `top-left`, `top-right`, `bottom-left`, `bottom-right`

## CLI Commands

```bash
veac build main.veac -o output.mp4   # Compile and render
veac check main.veac                  # Validate only
veac plan main.veac                   # Show FFmpeg commands
veac fmt main.veac                    # Format source
veac probe ./assets/intro.mp4         # Show media info
```

## Design Principles

- **Declarative**: Describe what the video should be, not how to process it.
- **Non-Turing-complete**: Variables and references only. No loops, conditionals, or recursion.
- **Agent-friendly**: Clear syntax and helpful error messages for AI-assisted editing.
- **Single source format**: `.veac` files are the canonical representation.
