# Language Reference

## Overview
VEAC (Video Editing as Code) is a declarative, non-Turing-complete domain-specific language for video editing. It compiles `.veac` source files into FFmpeg commands.

## Design Principles
- **Declarative**: Describe what the video should be, not how to process it
- **Non-Turing-complete**: Variables and references only — no loops, conditionals, or recursion
- **Agent-friendly**: Clear syntax and helpful errors for AI-assisted editing
- **Single source format**: `.veac` files are the canonical representation

## File Structure
```
my-project/
├── main.veac
├── shared.veac        # Optional: included modules
└── assets/
    ├── intro.mp4
    ├── bgm.mp3
    └── logo.png
```

## Language Elements

A `.veac` file consists of top-level declarations:

| Declaration | Purpose | Reference |
|---|---|---|
| `project` | Output configuration | [Project](project.md) |
| `asset` | Media file references | [Assets](assets.md) |
| `let` | Variable bindings | [Variables & Includes](variables-and-includes.md) |
| `include` | Module imports | [Variables & Includes](variables-and-includes.md) |
| `timeline` | Video composition | [Timeline & Tracks](timeline-and-tracks.md) |

## Track Items

Within timelines, different track types support different items:

| Item | Tracks | Reference |
|---|---|---|
| `clip` | video, audio | [Clips](clips.md) |
| `transition` | video | [Transitions](transitions.md) |
| `text` | text | [Overlays](overlays.md) |
| `image` | overlay | [Overlays](overlays.md) |
| `pip` | overlay | [Overlays](overlays.md) |
| `subtitle` | overlay | [Overlays](overlays.md) |
| `gap` | video, audio | [Overlays](overlays.md) |
| `freeze` | video | [Overlays](overlays.md) |

## Literal Types
See [Literals](literals.md) for time, color, and number formats.
