# Overlays

Overlay items are positioned on top of the video timeline using absolute time coordinates.

## Text Overlay

### Syntax
```veac
track text {
    text "content" {
        at       = 1s
        duration = 4s
        font     = "Arial"
        size     = 64
        color    = #FFFFFF
        position = "center"
    }
}
```

### Properties

| Property | Type | Default | Description |
|---|---|---|---|
| `at` | time | `0s` | When the text appears |
| `duration` | time | `5s` | How long the text is visible |
| `font` | string | `"Arial"` | Font family name |
| `size` | integer | `24` | Font size in pixels |
| `color` | color | `#FFFFFF` | Text color |
| `position` | string | `"center"` | Text position on screen |
| `fade_in` | time | — | Fade-in duration |
| `fade_out` | time | — | Fade-out duration |

## Image Overlay

### Syntax
```veac
track overlay {
    image asset_name {
        at       = 0s
        duration = 30s
        position = "top-right"
        scale    = 0.5
        opacity  = 0.6
    }
}
```

### Properties

| Property | Type | Default | Range | Description |
|---|---|---|---|---|
| `at` | time | `0s` | ≥ 0 | When the image appears |
| `duration` | time | `5s` | ≥ 0 | How long the image is visible |
| `position` | string | `"top-right"` | 9 positions | Image position on screen |
| `scale` | float | — | 0.0 – 10.0 | Scale factor |
| `opacity` | float | — | 0.0 – 1.0 | Opacity level |

## Picture-in-Picture (PIP)

### Syntax
```veac
track overlay {
    pip asset_name {
        from     = 0s
        to       = 60s
        at       = 0s
        duration = 60s
        position = "bottom-right"
        scale    = 0.25
    }
}
```

### Properties

| Property | Type | Default | Description |
|---|---|---|---|
| `from` | time | — | Source video start time |
| `to` | time | — | Source video end time |
| `at` | time | `0s` | When PIP appears on timeline |
| `duration` | time | `5s` | How long PIP is visible |
| `position` | string | `"bottom-right"` | PIP position |
| `scale` | float | `0.25` | Scale factor |

## Subtitle

### Syntax
```veac
track overlay {
    subtitle "path/to/subs.srt" {}
}
```
- Imports an SRT subtitle file
- Subtitles are rendered onto the video

## Gap

### Syntax
```veac
track video {
    gap { duration = 2s }
}
```
- Inserts a silent, black gap in the timeline
- `duration` is the only property (required)

## Freeze Frame

### Syntax
```veac
track video {
    freeze {
        at       = 5s
        duration = 3s
    }
}
```
- Holds a single frame from the preceding clip
- `at`: the time point in the previous clip to freeze
- `duration`: how long to hold the frame

## Position Values

All position properties accept these 9 values:

| Value | Description |
|---|---|
| `"center"` | Center of the frame |
| `"top"` | Top center |
| `"bottom"` | Bottom center |
| `"left"` | Left center |
| `"right"` | Right center |
| `"top-left"` | Top-left corner |
| `"top-right"` | Top-right corner |
| `"bottom-left"` | Bottom-left corner |
| `"bottom-right"` | Bottom-right corner |
