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
enum ProgrammError {
    FileError(io::Error),
    ParseError(serde_json::Error),
    MissingArgument(String),
    WriteError(WriteError),
}

impl From<io::Error> for ProgrammError {
    fn from(value: io::Error) -> Self {
        Self::FileError(value)
    }
}

impl From<serde_json::Error> for ProgrammError {
    fn from(value: serde_json::Error) -> Self {
        Self::ParseError(value)
    }
}

impl From<WriteError> for ProgrammError {
    fn from(value: WriteError) -> Self {
        ProgrammError::WriteError(value)
    }
}

fn main() -> Result<(), ProgrammError> {
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
        tables: vec![
            Box::new(svg),
            Box::new(cmap),
            Box::new(OS2 {
                avg_glyph_width: 100,
                weight_class: 400,
                width_class: 5,
                subscript: Script {
                    x_size: 60,
                    y_size: 60,
                    x_offset: 5,
                    y_offset: 40,
                },
                superscript: Script {
                    x_size: 60,
                    y_size: 60,
                    x_offset: 5,
                    y_offset: -40,
                },
                strikeout_size: 10,
                strikeout_position: 45,
                panose: Panose::default(),
                typo_ascender: 20,
                typo_descender: 30,
                typo_line_gap: 120,
                win_ascent: 60,
                win_descent: 80,
                x_height: 60,
                cap_height: 80,
                default_cahr: 0,
                break_char: ' ' as u16,
                max_context: 1,
            }),
        ],
    };

    let path = env::args().nth(2).unwrap_or("./out.otf".to_string());
    let mut file = File::create(path)?;

    document.write(&mut file)?;

    return Ok(());
}

fn read_manifest() -> Result<Manifest, ProgrammError> {
    let path = env::args()
        .nth(1)
        .ok_or(ProgrammError::MissingArgument("Manifest Path".to_string()))?;
    let file = fs::read(path)?;

    Ok(serde_json::de::from_slice(&file)?)
}
