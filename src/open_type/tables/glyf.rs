use std::cell::RefMut;

use crate::{
    layout::{Reservation, SeekWrite},
    open_type::{
        true_type::{Contour, Instrution},
        LayoutableTable, LayoutedTable,
    },
    Layoutable, Layouted,
};

#[derive(Debug, Clone)]
pub enum Glyph {
    Simple {
        contours: Vec<Contour>,
        instructions: Vec<Instrution>,
    },
}

pub struct Glyf {
    pub glyphs: Vec<Glyph>,
}

impl Layoutable<Box<dyn LayoutedTable>> for Glyf {
    fn layout(&self, layouter: &mut crate::Layouter) -> Box<dyn LayoutedTable> {
        let glyphs: Vec<_> = self
            .glyphs
            .iter()
            .map(|glyph| -> Box<dyn LayoutedGlyph> {
                match glyph {
                    Glyph::Simple {
                        contours,
                        instructions,
                    } => Box::new(LayoutedSimpleGlyph::new(contours, instructions)),
                }
            })
            .collect();

        let total_size = glyphs.iter().map(|g| g.size()).sum();

        Box::new(LayoutedGlyf {
            reservation: layouter.reserve(total_size),
            requires_another_pass: true,
            glyphs,
        })
    }
}

impl LayoutableTable for Glyf {
    fn tag(&self) -> [u8; 4] {
        *b"glyf"
    }
}

struct LayoutedGlyf {
    reservation: Reservation,
    requires_another_pass: bool,
    glyphs: Vec<Box<dyn LayoutedGlyph>>,
}

impl Layouted for LayoutedGlyf {
    fn reservation(&self) -> &Reservation {
        &self.reservation
    }

    fn requires_another_pass(&self) -> bool {
        self.requires_another_pass
    }

    fn pass(&mut self, _current_file: &[u8]) -> Result<(), crate::LayoutError> {
        self.requires_another_pass = false;

        let mut writer = self.reservation.writer();

        for glyph in self.glyphs.iter_mut() {
            glyph.write(&mut writer)?;
        }

        Ok(())
    }
}

impl LayoutedTable for LayoutedGlyf {
    fn tag(&self) -> [u8; 4] {
        *b"glyf"
    }
}

trait LayoutedGlyph {
    fn size(&self) -> usize;
    fn write(&self, writer: &mut RefMut<dyn SeekWrite>) -> std::io::Result<()>;
}

struct LayoutedSimpleGlyph {
    size: usize,
    contours: Vec<Contour>,
    instructions: Vec<Instrution>,
    x_min: i16,
    y_min: i16,
    x_max: i16,
    y_max: i16,
}

impl LayoutedSimpleGlyph {
    fn new(contours: &[Contour], instructions: &[Instrution]) -> Self {
        #[derive(Debug, Default, Clone, Copy)]
        struct Envelop {
            x_min: i16,
            y_min: i16,
            x_max: i16,
            y_max: i16,
        }

        let envelop = contours.iter().flat_map(|c| c.points.iter()).fold(
            Envelop::default(),
            |state, point| Envelop {
                x_min: state.x_min.min(point.x),
                y_min: state.y_min.min(point.y),
                x_max: state.x_max.max(point.x),
                y_max: state.y_max.max(point.y),
            },
        );

        LayoutedSimpleGlyph {
            size: 14
                + (contours.len() * 2)
                + (instructions.len() * 1)
                + (contours.iter().map(|c| c.points.len()).sum::<usize>() * 5),
            contours: contours.to_vec(),
            instructions: instructions.to_vec(),
            x_min: envelop.x_min,
            y_min: envelop.y_min,
            x_max: envelop.x_max,
            y_max: envelop.y_max,
        }
    }
}

impl LayoutedGlyph for LayoutedSimpleGlyph {
    fn size(&self) -> usize {
        self.size
    }

    fn write(&self, writer: &mut RefMut<dyn SeekWrite>) -> std::io::Result<()> {
        use byteorder::{WriteBytesExt, BE};

        writer.write_u16::<BE>(self.contours.len() as u16)?;

        writer.write_i16::<BE>(self.x_min)?;
        writer.write_i16::<BE>(self.y_min)?;
        writer.write_i16::<BE>(self.x_max)?;
        writer.write_i16::<BE>(self.y_max)?;

        let mut pts_offset = 0;
        for contour in self.contours.iter() {
            pts_offset += contour.points.len() as u16;

            writer.write_u16::<BE>(pts_offset)?;
        }

        writer.write_u16::<BE>(self.instructions.len() as u16)?;

        for _instruction in self.instructions.iter() {
            todo!()
        }

        // Flags
        for contour in self.contours.iter() {
            for point in contour.points.iter() {
                let mut flags = 0;

                if point.is_on_curve {
                    flags += 1
                }

                writer.write_u8(flags)?;
            }
        }

        // x-Coordinates
        for contour in self.contours.iter() {
            for point in contour.points.iter() {
                writer.write_i16::<BE>(point.x)?;
            }
        }

        // y-Coordinates
        for contour in self.contours.iter() {
            for point in contour.points.iter() {
                writer.write_i16::<BE>(point.y)?;
            }
        }

        Ok(())
    }
}
