use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub name: String,
    pub glyphs: Vec<GlyphRange>,
}

#[derive(Debug, Deserialize)]
pub struct GlyphRange {
    pub start: char,
    pub end: char,
    pub file: String,
}
