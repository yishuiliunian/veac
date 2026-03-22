# Literals

VEAC supports several literal types for expressing values in the language.

## Time Literals

Time values can be expressed in multiple formats:

### Seconds
```veac
3.5s      // 3.5 seconds
0s        // zero
10s       // 10 seconds
```
- Suffix: `s`
- Accepts integers and floating-point numbers

### Milliseconds
```veac
500ms     // 500 milliseconds (= 0.5s)
1500ms    // 1500 milliseconds (= 1.5s)
```
- Suffix: `ms`
- Accepts integers and floating-point numbers
- Internally converted to seconds

### Frames
```veac
84f       // 84 frames
1f        // 1 frame
```
- Suffix: `f`
- Accepts integers only
- Converted to seconds using the project's `fps` setting (e.g., 84f at 30fps = 2.8s)

### SMPTE Timecode
```veac
"00:01:30:12"    // 1 minute, 30 seconds, 12 frames
```
- Format: `"HH:MM:SS:FF"` (hours:minutes:seconds:frames)
- Must be quoted as a string
- Frame component is converted using the project's `fps` setting
- HH: 00–23, MM: 00–59, SS: 00–59, FF: 00–99

### Bare Numbers
```veac
3.5       // 3.5 (interpreted as seconds in time contexts)
10        // 10 (integer)
```
- Bare numbers without a suffix are interpreted based on context

## Color Literals

Colors are expressed as hexadecimal values prefixed with `#`:

### RGB (6-digit)
```veac
#FFFFFF    // White
#000000    // Black
#FF5500    // Orange
```
- Format: `#RRGGBB`
- Case-insensitive (`#ffffff` = `#FFFFFF`)

### RGBA (8-digit)
```veac
#FFFFFF80    // White at 50% opacity
#FF000040    // Red at 25% opacity
```
- Format: `#RRGGBBAA`
- `AA` = alpha channel (00 = transparent, FF = opaque)

## Number Literals

### Integers
```veac
42        // Positive integer
0         // Zero
```

### Floating-Point
```veac
3.14      // Float
0.8       // Float
-0.5      // Negative float
```

## String Literals

Strings are enclosed in double quotes:
```veac
"Hello World"
"assets/intro.mp4"
"00:01:30:12"         // Also used for SMPTE timecode
"1920x1080"           // Resolution format
```

## Boolean Literals

```veac
true
false
```
Used for properties like `reverse`, `normalize`, and `stabilize`.

## Summary Table

| Type | Examples | Usage |
|---|---|---|
| Seconds | `3.5s`, `0s`, `10s` | Time properties (from, to, duration, at, fade_in, etc.) |
| Milliseconds | `500ms`, `1500ms` | Time properties |
| Frames | `84f`, `1f` | Time properties (converted via fps) |
| SMPTE | `"00:01:30:12"` | Time properties |
| Color (RGB) | `#FFFFFF`, `#FF5500` | color property |
| Color (RGBA) | `#FFFFFF80` | color property with alpha |
| Integer | `42`, `0` | size, fps, loop, etc. |
| Float | `3.14`, `0.8` | volume, speed, brightness, etc. |
| String | `"text"` | Content, paths, resolution, codec, etc. |
| Boolean | `true`, `false` | reverse, normalize, stabilize |
