use std::{
    env,
    fs::{self, File},
    io,
};

use manifest::Manifest;
use open_type::*;

mod manifest;
mod open_type;

#[derive(Debug)]
enum Error {
    FileError(io::Error),
    ParseError(serde_json::Error),
    MissingArgument(String),
    WriteError(WriteError),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::FileError(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::ParseError(value)
    }
}

impl From<WriteError> for Error {
    fn from(value: WriteError) -> Self {
        Error::WriteError(value)
    }
}

fn main() -> Result<(), Error> {
    let manifest = read_manifest()?;

    let mut svg = SVG { documents: vec![] };
    let mut cmap = CMap { ranges: vec![] };

    let mut next_free_id = 1;

    for range in manifest.glyphs {
        let start_code = range.start as u16;
        let end_code = range.end as u16;
        let num_glyphs = end_code - start_code + 1;

        cmap.ranges.push(CharacterRange {
            start: start_code,
            end: end_code,
            start_id: next_free_id,
        });

        svg.documents.push({
            SVGDocumentRecord {
                start_glyph_id: next_free_id,
                end_glyph_id: next_free_id + num_glyphs - 1,
                document: range.file,
            }
        });

        next_free_id += num_glyphs;
    }

    let mut document = Document {
        sfnt_version: SFNTVersion::OpenType,
        tables: vec![Box::new(svg), Box::new(cmap)],
    };

    let path = env::args().nth(2).unwrap_or("./out.otf".to_string());
    let mut file = File::create(path)?;

    document.store(&mut file)?;

    return Ok(());
}

fn read_manifest() -> Result<Manifest, Error> {
    let path = env::args()
        .nth(1)
        .ok_or(Error::MissingArgument("Manifest Path".to_string()))?;
    let file = fs::read(path)?;
    let result = serde_json::de::from_slice(&file)?;
    return Ok(result);
}
