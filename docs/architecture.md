# Architecture

## Overview
VEAC uses a traditional compiler pipeline: source code → lexer → parser → semantic analysis → IR → code generation → runtime execution.

The project is organized as a Cargo workspace with 4 crates:

```
veac/
├── crates/
│   ├── veac-lang/       # Frontend: lexer, parser, semantic analyzer, IR
│   ├── veac-codegen/    # Backend: IR → FFmpeg commands + filter graphs
│   ├── veac-runtime/    # Execution: FFmpeg process management, media probing
│   └── veac-cli/        # CLI: command-line interface entry point
```

## Compilation Pipeline

```
.veac source
    │
    ▼
┌─────────┐
│  Lexer   │  Tokenizes source text into tokens
└────┬─────┘  (veac-lang/src/lexer/)
     │
     ▼
┌─────────┐
│ Parser   │  Builds AST from token stream
└────┬─────┘  (veac-lang/src/parser/)
     │
     ▼
┌──────────┐
│ Semantic  │  Validates, resolves references, type-checks
│ Analyzer  │  Processes includes (with cycle detection)
└────┬──────┘  (veac-lang/src/semantic/)
     │
     ▼
┌─────────┐
│   IR     │  Intermediate representation (resolved, typed)
└────┬─────┘  (veac-lang/src/ir/)
     │
     ▼
┌──────────┐
│ Codegen   │  Generates FFmpeg commands + filter graphs
└────┬──────┘  (veac-codegen/src/)
     │
     ▼
┌──────────┐
│ Runtime   │  Executes FFmpeg, tracks progress, probes media
└────┬──────┘  (veac-runtime/src/)
     │
     ▼
  output.mp4
```

## Crate Details

### veac-lang (Frontend)
- **Lexer** (`lexer/mod.rs` + `lexer/scan.rs`): Tokenizes source text. Handles time literals (3.5s, 500ms, 84f, SMPTE), color literals (#FFFFFF), strings, numbers, identifiers, keywords.
- **Parser** (`parser/mod.rs` + `parser/timeline.rs`): Recursive descent parser. `mod.rs` handles top-level declarations (project, asset, let, include, timeline). `timeline.rs` handles track items (clip, transition, text, image, gap, freeze, pip, subtitle).
- **Semantic Analyzer** (`semantic/mod.rs` + `semantic/resolve.rs` + specialized resolvers): Validates all values against allowed ranges, resolves variable references, processes includes with cycle detection, performs type checking.
- **IR** (`ir/`): Fully resolved intermediate representation. All time values converted to seconds (f64). All variable references resolved to concrete values.

### veac-codegen (Backend)
- Converts IR to FFmpeg CLI commands
- Builds complex filter graphs for effects, transitions, overlays
- Maps 21 clip effects to FFmpeg filters (eq, zoompan, boxblur, etc.)
- Maps 20 transition types to FFmpeg xfade filter
- Handles text overlay via drawtext filter, image overlay via overlay filter

### veac-runtime (Execution)
- Spawns and manages FFmpeg processes
- Tracks rendering progress
- Media probing (resolution, duration, codec, etc.)
- Error reporting from FFmpeg stderr

### veac-cli (CLI Entry)
- 6 commands: build, check, plan, fmt, probe, batch
- Uses clap for argument parsing
- Orchestrates the pipeline: parse → analyze → codegen → execute

## Design Decisions
- **Non-Turing-complete**: Only variables + references, no loops/conditionals/recursion. This ensures predictable compilation and makes the language agent-friendly.
- **File splitting**: Files target <200 lines, split by responsibility (e.g., lexer core vs. token scanning).
- **Sequential compilation**: No parallel compilation needed — single-file projects compile fast enough.
