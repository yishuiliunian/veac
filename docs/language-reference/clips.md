# Clip Properties

Clips are the primary building blocks of a timeline. They reference an asset and optionally apply effects.

## Syntax
```veac
clip asset_name {
    property = value
    ...
}
```

## Time Properties

| Property | Type | Description |
|---|---|---|
| `from` | time | Source start time (must be < `to`) |
| `to` | time | Source end time (must be > `from`) |
| `duration` | time | Alternative to `to`: specifies clip duration |

## Audio Properties

| Property | Type | Range | Default | Description |
|---|---|---|---|---|
| `volume` | float | 0.0 – 1.0 | 1.0 | Audio volume multiplier |
| `speed` | float | >0.0 – 100.0 | 1.0 | Playback speed multiplier |
| `fade_in` | time | ≥ 0 | — | Audio/video fade-in duration |
| `fade_out` | time | ≥ 0 | — | Audio/video fade-out duration |

## Color Grading

| Property | Type | Range | Default | Description |
|---|---|---|---|---|
| `brightness` | float | -1.0 – 1.0 | 0.0 | Brightness adjustment |
| `contrast` | float | 0.0 – 3.0 | 1.0 | Contrast adjustment |
| `saturation` | float | 0.0 – 3.0 | 1.0 | Color saturation |

## Transform Effects

| Property | Type | Range | Default | Description |
|---|---|---|---|---|
| `zoom` | float | 1.0 – 10.0 | — | Ken Burns zoom effect (max zoom level) |
| `crop` | string | — | — | Crop region: `"WIDTHxHEIGHT+X+Y"` |
| `blur` | float | 0.0 – 100.0 | — | Blur intensity |
| `opacity` | float | 0.0 – 1.0 | 1.0 | Opacity (0 = transparent, 1 = opaque) |
| `rotate` | float | any | — | Rotation angle in degrees |
| `flip` | string | `"horizontal"`, `"vertical"`, `"both"` | — | Flip direction |
| `vignette` | float | 0.0 – 1.0 | — | Vignette effect intensity |
| `grain` | float | 0.0 – 1.0 | — | Film grain intensity |
| `sharpen` | float | > 0.0 | — | Sharpening intensity |
| `pan_x` | float | -1.0 – 1.0 | 0.0 | Horizontal pan focus (-1=left, 1=right) |
| `pan_y` | float | -1.0 – 1.0 | 0.0 | Vertical pan focus (-1=top, 1=bottom) |

## Advanced Properties

| Property | Type | Values | Description |
|---|---|---|---|
| `reverse` | boolean | `true`/`false` | Reverse playback |
| `chromakey` | string | `"green"`, `"blue"`, `"#RRGGBB"` | Chroma key (green/blue screen removal) |
| `normalize` | boolean | `true`/`false` | Apply audio loudness normalization |
| `loop` | integer | ≥ 1 | Loop the clip N times |
| `stabilize` | boolean | `true`/`false` | Apply video stabilization |

## Example

```veac
clip scene1 {
    from       = 0s
    to         = 12s
    speed      = 2.0
    brightness = 0.06
    contrast   = 1.15
    saturation = 1.3
    fade_in    = 1s
    fade_out   = 2s
    volume     = 0.8
}
```

## Validation Rules
- `from` must be less than `to` (when both specified)
- `fade_in` + `fade_out` must not exceed clip duration
- Values must be within their documented ranges
- The referenced asset must be declared
