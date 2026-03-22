# Project Declaration

The `project` block defines output configuration for the video.

## Syntax

```veac
project "name" {
    resolution = "1920x1080"
    fps        = 30
    format     = "mp4"
    codec      = "h264"
    quality    = "high"
    fit        = "fill"
}
```

## Fields

| Field | Type | Default | Description |
|---|---|---|---|
| `resolution` | string | `"1920x1080"` | Output resolution in `"WIDTHxHEIGHT"` format |
| `fps` | integer | `30` | Frames per second |
| `format` | string | `"mp4"` | Output container format |
| `codec` | string | `"h264"` | Video codec |
| `quality` | string | `"high"` | Encoding quality preset |
| `fit` | string | `"fill"` | How to fit source content to output resolution |

## Format Values

| Value | Extension | Description |
|---|---|---|
| `"mp4"` | `.mp4` | MPEG-4 container (most compatible) |
| `"mkv"` | `.mkv` | Matroska container |
| `"webm"` | `.webm` | WebM container |
| `"mov"` | `.mov` | QuickTime container |

## Codec Values

| Value | FFmpeg Encoder | Description |
|---|---|---|
| `"h264"` | `libx264` | H.264/AVC (most compatible) |
| `"h265"` | `libx265` | H.265/HEVC (better compression) |
| `"vp9"` | `libvpx-vp9` | VP9 (WebM) |
| `"av1"` | `libaom-av1` | AV1 (best compression, slowest) |

## Quality Values

| Value | Preset | CRF | Description |
|---|---|---|---|
| `"low"` | ultrafast | 28 | Fast encoding, larger file |
| `"medium"` | medium | 23 | Balanced |
| `"high"` | slow | 18 | High quality, slower encoding |
| `"lossless"` | veryslow | 0 | Lossless, largest file |

## Fit Values

| Value | Description |
|---|---|
| `"fill"` | Scale to fill the frame (may crop) |
| `"letterbox"` | Scale to fit with black bars |
| `"crop"` | Center-crop to fill |

## Rules
- Exactly one `project` block is required per compilation unit
- The `name` string is used for logging and identification only
