# VEAC — Video Editing as Code

![CI](https://github.com/AstroStone/veac/actions/workflows/ci.yml/badge.svg)
![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)

A declarative, non-Turing-complete domain-specific language that compiles `.veac` source files into FFmpeg commands to produce video output.

## Quick Example

```veac
project "hello" {
    resolution = "1920x1080"
    fps = 30
    format = "mp4"
}

asset intro = video("assets/intro.mp4")

timeline main {
    track video {
        clip intro { from = 0s  to = 10s }
    }
}
```

## Features

- 21 video/audio effects (brightness, contrast, speed, zoom, blur, etc.)
- 20 transition types (fade, dissolve, wipe, slide, etc.)
- Text, image, PIP, subtitle overlays
- Variable system with `let` bindings
- Module system with `include` for reuse
- Multiple time formats: seconds (`3.5s`), milliseconds (`500ms`), frames (`84f`), SMPTE timecode
- Built-in formatter (`veac fmt`)
- Batch rendering from CSV templates
- Dry-run mode (`veac plan`) to preview FFmpeg commands

## Installation

```bash
# Prerequisites: Rust toolchain and FFmpeg
git clone https://github.com/AstroStone/veac.git
cd veac
cargo install --path crates/veac-cli
```

## Quick Start

```bash
veac build main.veac -o output.mp4   # Compile and render
veac check main.veac                  # Validate only
veac plan main.veac                   # Show FFmpeg commands (dry run)
veac fmt main.veac                    # Format source
veac probe assets/intro.mp4           # Show media info
veac batch template.veac --params data.csv -o out/  # Batch render
```

## Documentation

- [Getting Started](docs/getting-started.md)
- [CLI Reference](docs/cli-reference.md)
- [Language Reference](docs/language-reference/)
- [Architecture](docs/architecture.md)

## Examples

See the [`examples/`](examples/) directory:

- `hello-world/` — Minimal single-clip project
- `minimal/` — Basic project structure
- `text-overlay/` — Text overlays demo
- `transitions/` — Transition effects
- `image-overlay/` — Image watermark
- `speed-demo/` — Speed control
- `color-grade/` — Color grading effects
- `batch-demo/` — Batch rendering from CSV
- `all-features/` — Comprehensive demo with all capabilities

## Design Principles

- **Declarative**: Describe what the video should be, not how to process it
- **Non-Turing-complete**: Variables and references only — no loops, conditionals, or recursion
- **Agent-friendly**: Clear syntax and helpful error messages for AI-assisted editing
- **Single source format**: `.veac` files are the canonical representation

## License

MIT — see [LICENSE](LICENSE)

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md)
