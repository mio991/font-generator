use byteorder::{WriteBytesExt, BE};

use super::super::{Tag, WriteError};
use super::table::Table;

const TAG: Tag = *b"SVG ";

#[derive(Debug)]
pub struct SVG {
    pub documents: Vec<SVGDocumentRecord>,
}

impl Table for SVG {
    fn get_tag(&self) -> Tag {
        TAG
    }

    fn store_internal(&self, writer: &mut dyn std::io::Write) -> Result<(), WriteError> {
        writer.write_u16::<BE>(0)?; // Version
        writer.write_u32::<BE>(10)?; // Offset
        writer.write_u32::<BE>(0)?; // Reserved^

        let num_documents = self.documents.len() as u16;
        writer.write_u16::<BE>(num_documents)?; // numEntries

        let mut blobs = Vec::<Vec<u8>>::new();
        let blobs_offset = 2 + (num_documents as u32) * 12;

        let mut documents = self.documents.clone();
        documents.sort_by_key(|a| a.start_glyph_id);

        for document in documents.iter() {
            let offset = blobs_offset + blobs.iter().map(|t| t.len() as u32).sum::<u32>();
            // writes 12 Bytes
            let blob = document.store(writer, offset)?;

            blobs.push(blob)
        }

        for table in blobs {
            writer.write_all(&table)?;
        }

        return Ok(());
    }
}

#[derive(Debug, Clone)]
pub struct SVGDocumentRecord {
    pub start_glyph_id: u16,
    pub end_glyph_id: u16,
    pub document: String,
}

impl SVGDocumentRecord {
    fn store(&self, writer: &mut dyn std::io::Write, offset: u32) -> Result<Vec<u8>, WriteError> {
        let file = std::fs::read(&self.document)?;

        writer.write_u16::<BE>(self.start_glyph_id)?; // startGlyphID
        writer.write_u16::<BE>(self.end_glyph_id)?; // endGlyphID
        writer.write_u32::<BE>(offset)?; // Offset
        writer.write_u32::<BE>(file.len() as u32)?; // length

        return Ok(file);
    }
}
