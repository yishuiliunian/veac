use std::path::PathBuf;

/// A complete VEAC program.
#[derive(Debug, Clone)]
pub struct Program {
    pub includes: Vec<IncludeDecl>,
    pub project: Option<ProjectDecl>,
    pub assets: Vec<AssetDecl>,
    pub variables: Vec<LetDecl>,
    pub timelines: Vec<TimelineDecl>,
    pub outputs: Vec<OutputDecl>,
}

/// `project "name" { ... }`
#[derive(Debug, Clone)]
pub struct ProjectDecl {
    pub name: String,
    pub attributes: Vec<Attribute>,
}

/// `asset name = video("path")`
#[derive(Debug, Clone)]
pub struct AssetDecl {
    pub name: String,
    pub kind: AssetKind,
    pub path: PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AssetKind {
    Video,
    Audio,
    Image,
}

/// `let name = expr`
#[derive(Debug, Clone)]
pub struct LetDecl {
    pub name: String,
    pub value: Expression,
}

/// `timeline name { ... }`
#[derive(Debug, Clone)]
pub struct TimelineDecl {
    pub name: String,
    pub tracks: Vec<TrackDecl>,
}

/// `track video { ... }` or `track audio { ... }` or `track text { ... }`
#[derive(Debug, Clone)]
pub struct TrackDecl {
    pub kind: TrackKind,
    pub items: Vec<TrackItem>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TrackKind {
    Video,
    Audio,
    Text,
    Overlay,
}

/// Items within a track.
#[derive(Debug, Clone)]
pub enum TrackItem {
    Clip(ClipDecl),
    TextOverlay(TextOverlayDecl),
    Transition(TransitionDecl),
    ImageOverlay(ImageOverlayDecl),
    Gap(GapDecl),
    Freeze(FreezeDecl),
    Pip(PipDecl),
    Subtitle(SubtitleDecl),
}

/// `clip asset_ref { ... }`
#[derive(Debug, Clone)]
pub struct ClipDecl {
    pub asset_ref: String,
    pub attributes: Vec<Attribute>,
}

/// `text "content" { ... }`
#[derive(Debug, Clone)]
pub struct TextOverlayDecl {
    pub content: String,
    pub attributes: Vec<Attribute>,
}

/// A key = value pair inside a block.
#[derive(Debug, Clone)]
pub struct Attribute {
    pub key: String,
    pub value: Expression,
}

/// Expression types in VEAC.
#[derive(Debug, Clone)]
pub enum Expression {
    StringLit(String),
    IntLit(i64),
    FloatLit(f64),
    BoolLit(bool),
    TimeSec(f64),
    TimeMs(f64),
    TimeFrames(u64),
    Smpte(String),
    ColorLit(String),
    Ident(String),
}

/// `transition { type = "fade"  duration = 1s }`
#[derive(Debug, Clone)]
pub struct TransitionDecl {
    pub attributes: Vec<Attribute>,
}

/// `image asset_ref { at = 0s  duration = 30s  position = "top-right" ... }`
#[derive(Debug, Clone)]
pub struct ImageOverlayDecl {
    pub asset_ref: String,
    pub attributes: Vec<Attribute>,
}

/// `include "./templates/intro-template.veac"`
#[derive(Debug, Clone)]
pub struct IncludeDecl {
    pub path: PathBuf,
}

/// `gap { duration = 2s }` — silent black gap in the timeline.
#[derive(Debug, Clone)]
pub struct GapDecl {
    pub attributes: Vec<Attribute>,
}

/// `freeze asset_ref { duration = 3s }` — freeze the last frame of a clip.
#[derive(Debug, Clone)]
pub struct FreezeDecl {
    pub asset_ref: String,
    pub attributes: Vec<Attribute>,
}

/// `pip asset_ref { ... }` — picture-in-picture overlay.
#[derive(Debug, Clone)]
pub struct PipDecl {
    pub asset_ref: String,
    pub attributes: Vec<Attribute>,
}

/// `subtitle "path.srt" { ... }` — subtitle import.
#[derive(Debug, Clone)]
pub struct SubtitleDecl {
    pub path: PathBuf,
    pub attributes: Vec<Attribute>,
}

/// `output "path" { ... }` — multi-output definition.
#[derive(Debug, Clone)]
pub struct OutputDecl {
    pub path: PathBuf,
    pub attributes: Vec<Attribute>,
}
