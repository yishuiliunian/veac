# Asset Declarations

Assets declare references to media files used in the timeline.

## Syntax

```veac
asset name = video("path/to/file.mp4")
asset name = audio("path/to/file.mp3")
asset name = image("path/to/file.png")
```

## Asset Types

| Type | Function | Supported Formats |
|---|---|---|
| Video | `video("path")` | Any format FFmpeg supports (mp4, mkv, webm, mov, avi, etc.) |
| Audio | `audio("path")` | Any format FFmpeg supports (mp3, wav, aac, m4a, flac, ogg, etc.) |
| Image | `image("path")` | Any format FFmpeg supports (png, jpg, gif, bmp, webp, etc.) |

## Path Resolution
- Paths are resolved **relative to the `.veac` file** containing the declaration
- Both forward slashes (`/`) and backslashes (`\`) are supported
- Paths must be quoted strings

## Examples

```veac
// Video assets
asset intro = video("assets/intro.mp4")
asset outro = video("../shared/outro.mkv")

// Audio assets
asset bgm = audio("assets/bgm.m4a")
asset sfx = audio("assets/click.wav")

// Image assets
asset logo = image("assets/logo.png")
asset bg = image("assets/background.jpg")
```

## Rules
- Asset names must be unique within the compilation unit (including included files)
- The referenced file does not need to exist at check time (`veac check`), but must exist at build time (`veac build`)
- Asset names are used as references in `clip`, `image`, and `pip` items
