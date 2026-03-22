use std::path::Path;

use veac_lang::error::VeacError;
use veac_lang::ir::{IrProgram, IrTrackItem};
use veac_lang::lexer::Lexer;
use veac_lang::parser::Parser;
use veac_lang::semantic::SemanticAnalyzer;

/// Run the frontend pipeline: lex -> parse -> analyze.
/// Returns the validated IR on success.
pub fn compile(source: &str, file: &Path) -> Result<IrProgram, PipelineError> {
    let filename = file.display().to_string();
    let base_dir = file.parent().unwrap_or(Path::new("."));

    // Lexing
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().map_err(|e| PipelineError {
        formatted: e.format(source, &filename),
        source_error: e,
    })?;

    // Parsing
    let mut parser = Parser::new(tokens);
    let program = parser.parse().map_err(|e| PipelineError {
        formatted: e.format(source, &filename),
        source_error: e,
    })?;

    // Semantic analysis
    let analyzer = SemanticAnalyzer::new(base_dir);
    let ir = analyzer.analyze(&program).map_err(|e| PipelineError {
        formatted: e.format(source, &filename),
        source_error: e,
    })?;

    Ok(ir)
}

/// Wraps a VeacError with its pre-formatted display string.
#[derive(Debug)]
pub struct PipelineError {
    pub formatted: String,
    #[allow(dead_code)]
    pub source_error: VeacError,
}

impl std::fmt::Display for PipelineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted)
    }
}

impl std::error::Error for PipelineError {}

/// Probe each asset referenced in the IR and update `has_audio` on every clip.
/// Assets that cannot be probed (e.g. missing files during plan/check) are
/// left with the default `has_audio = true` so the generated command is
/// conservative (references `[idx:a]` as before).
pub fn probe_audio_streams(ir: &mut IrProgram, base_dir: &Path) {
    use std::collections::HashMap;

    // Build a name -> has_audio map by probing each asset once.
    let mut audio_map: HashMap<String, bool> = HashMap::new();
    for asset in &ir.assets {
        let path = if asset.path.is_relative() {
            base_dir.join(&asset.path)
        } else {
            asset.path.clone()
        };
        let has_audio = match veac_runtime::asset::probe(&path) {
            Ok(info) => info.has_audio,
            Err(_) => true, // conservative default: assume audio exists
        };
        audio_map.insert(asset.name.clone(), has_audio);
    }

    // Walk every clip in the timeline and stamp has_audio.
    for track in &mut ir.timeline.tracks {
        for item in &mut track.items {
            if let IrTrackItem::Clip(clip) = item {
                if let Some(&has) = audio_map.get(&clip.asset_name) {
                    clip.has_audio = has;
                }
            }
        }
    }
}
