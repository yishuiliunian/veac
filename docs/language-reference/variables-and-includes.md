# Variables and Includes

## Variables

The `let` keyword declares immutable variable bindings.

### Syntax

```veac
let name = expression
```

### Supported Types

Variables can hold any expression type:

```veac
let fade_duration = 1s          // Time (seconds)
let delay = 500ms               // Time (milliseconds)
let frame_count = 84f           // Time (frames)
let default_volume = 0.8        // Float
let repeat_count = 3            // Integer
let title = "Hello World"       // String
let accent_color = #FF5500      // Color
let is_enabled = true           // Boolean
```

### Type Inference
Types are inferred from the assigned expression. No explicit type annotations are needed or supported.

### Usage
Variables are referenced by name in any position where a value is expected:

```veac
let start = 0s
let end = 10s

timeline main {
    track video {
        clip intro { from = start  to = end }
    }
}
```

### Rules
- Variables are immutable — once declared, they cannot be reassigned
- Variables must be declared before use
- Variable names must be unique within scope (including included files)
- Variables from included files are available in the including file

---

## Include

The `include` statement imports declarations from another `.veac` file.

### Syntax

```veac
include "./path/to/file.veac"
```

### Behavior
- All `asset`, `let`, and track declarations from the included file are merged into the current compilation unit
- Paths are resolved relative to the file containing the `include` statement
- Includes are processed recursively (included files can include other files)
- Circular includes are detected and reported as errors

### Example

**shared_assets.veac**:
```veac
asset bgm = audio("assets/bgm.m4a")
asset logo = image("assets/logo.png")
let default_volume = 0.3
```

**main.veac**:
```veac
include "./shared_assets.veac"

project "my-video" {
    resolution = "1920x1080"
    fps = 30
    format = "mp4"
}

asset intro = video("assets/intro.mp4")

timeline main {
    track video {
        clip intro { from = 0s  to = 10s }
    }
    track audio {
        clip bgm { volume = default_volume }
    }
}
```
