# Transitions

Transitions create smooth visual effects between adjacent clips in a video track.

## Syntax
```veac
transition {
    type     = "fade"
    duration = 1s
}
```

## Properties

| Property | Type | Default | Description |
|---|---|---|---|
| `type` | string | `"fade"` | Transition effect type |
| `duration` | time | `1s` | Transition duration |

## Available Transition Types

VEAC supports 20+ transition types, all mapped to FFmpeg's `xfade` filter:

### Fade Effects
| Type | Description |
|---|---|
| `"fade"` | Standard cross-fade |
| `"fadeblack"` / `"fade-black"` | Fade through black |
| `"fadewhite"` / `"fade-white"` | Fade through white |
| `"dissolve"` | Dissolve blend |

### Wipe Effects
| Type | Description |
|---|---|
| `"wipe-left"` / `"wipeleft"` | Wipe from right to left |
| `"wipe-right"` / `"wiperight"` | Wipe from left to right |
| `"wipe-up"` / `"wipeup"` | Wipe from bottom to top |
| `"wipe-down"` / `"wipedown"` | Wipe from top to bottom |

### Slide Effects
| Type | Description |
|---|---|
| `"slide-left"` / `"slideleft"` | Slide in from right |
| `"slide-right"` / `"slideright"` | Slide in from left |
| `"slide-up"` / `"slideup"` | Slide in from bottom |
| `"slide-down"` / `"slidedown"` | Slide in from top |

### Smooth Effects
| Type | Description |
|---|---|
| `"smooth-left"` / `"smoothleft"` | Smooth transition left |
| `"smooth-right"` / `"smoothright"` | Smooth transition right |
| `"smooth-up"` / `"smoothup"` | Smooth transition up |
| `"smooth-down"` / `"smoothdown"` | Smooth transition down |

### Other Effects
| Type | Description |
|---|---|
| `"zoom-in"` / `"zoomin"` | Zoom into next clip |
| `"squeeze-h"` / `"squeezeh"` | Horizontal squeeze |
| `"squeeze-v"` / `"squeezev"` | Vertical squeeze |
| `"circlecrop"` / `"circle-crop"` | Circular crop reveal |
| `"pixelize"` | Pixelation transition |

## Naming Convention
Most transitions accept both hyphenated (`"wipe-left"`) and concatenated (`"wipeleft"`) forms.

## Example
```veac
track video {
    clip intro { from = 0s  to = 10s }
    transition { type = "dissolve"  duration = 1.5s }
    clip scene1 { from = 0s  to = 15s }
    transition { type = "wipe-left"  duration = 1s }
    clip outro { from = 0s  to = 8s }
}
```

## Rules
- Transitions must appear between two clips
- The transition duration creates an overlap between the adjacent clips
- Duration must be positive
