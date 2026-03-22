# Timeline and Tracks

## Timeline

The `timeline` block is the top-level container for video composition.

### Syntax
```veac
timeline name {
    track video { ... }
    track audio { ... }
    track text { ... }
    track overlay { ... }
}
```

### Rules
- At least one `timeline` block is required
- A timeline contains one or more `track` blocks

## Track Types

### video track
The primary video track. Contains clips, transitions, gaps, and freeze frames.
```veac
track video {
    clip intro { from = 0s  to = 10s }
    transition { type = "fade"  duration = 1s }
    clip scene1 { from = 0s  to = 15s  speed = 2.0 }
    gap { duration = 2s }
    freeze { at = 5s  duration = 3s }
}
```
- Clips play sequentially
- Transitions apply between adjacent clips (overlapping)
- Gaps insert silence/black
- Freeze holds the last frame of the previous clip

### audio track
Contains audio clips with volume and fade control.
```veac
track audio {
    clip bgm {
        volume   = 0.3
        fade_in  = 2s
        fade_out = 3s
    }
}
```

### text track
Contains text overlays positioned on the video.
```veac
track text {
    text "Title" {
        at       = 1s
        duration = 4s
        font     = "Arial"
        size     = 64
        color    = #FFFFFF
        position = "center"
    }
}
```

### overlay track
Contains image overlays, picture-in-picture, and subtitles.
```veac
track overlay {
    image logo {
        at       = 0s
        duration = 30s
        position = "top-right"
        scale    = 0.5
        opacity  = 0.6
    }
    pip camera {
        from     = 0s
        to       = 60s
        at       = 0s
        duration = 60s
        position = "bottom-right"
        scale    = 0.25
    }
    subtitle "subs.srt" {}
}
```

## Playback Model
- Items within a video/audio track play **sequentially**
- Transitions create an overlap between adjacent clips
- Overlay/text items use **absolute positioning** via the `at` property
- Multiple tracks are composited (layered) together
