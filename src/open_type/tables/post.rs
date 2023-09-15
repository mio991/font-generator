use crate::{
    layout::Reservation,
    open_type::{Fixed, LayoutableTable, LayoutedTable},
    Layoutable, Layouted,
};

#[derive(Debug, Clone)]
pub struct Post {
    /** Italic angle in counter-clockwise degrees from the vertical. Zero for upright text, negative for text that leans to the right (forward). */
    pub italic_angle: Fixed,
    /** This is the suggested distance of the top of the underline from the baseline (negative values indicate below baseline). The PostScript definition of this FontInfo dictionary key (the y coordinate of the center of the stroke) is not used for historical reasons. The value of the PostScript key may be calculated by subtracting half the underlineThickness from the value of this field. */
    pub underline_position: i16,
    /** Suggested values for the underline thickness. In general, the underline thickness should match the thickness of the underscore character (U+005F LOW LINE), and should also match the strikeout thickness, which is specified in the OS/2 table. */
    pub underline_thickness: i16,
    /** Set to 0 if the font is proportionally spaced, non-zero if the font is not proportionally spaced (i.e. monospaced). */
    pub is_fixed_pitch: bool,
    /** Minimum memory usage when an OpenType font is downloaded. */
    pub min_mem_type42: u32,
    /** Maximum memory usage when an OpenType font is downloaded. */
    pub max_mem_type42: u32,
    /** Minimum memory usage when an OpenType font is downloaded as a Type 1 font. */
    pub min_mem_type1: u32,
    /** Maximum memory usage when an OpenType font is downloaded as a Type 1 font. */
    pub max_mem_type1: u32,
}

impl Default for Post {
    fn default() -> Self {
        Self {
            italic_angle: Fixed { major: 0, minor: 0 },
            underline_position: 0,
            underline_thickness: 1,
            is_fixed_pitch: true,
            min_mem_type42: 0,
            max_mem_type42: 0,
            min_mem_type1: 0,
            max_mem_type1: 0,
        }
    }
}

impl Layoutable<Box<dyn LayoutedTable>> for Post {
    fn layout(&self, layouter: &mut crate::Layouter) -> Box<dyn LayoutedTable> {
        Box::new(LayoutedPost {
            reservation: layouter.reserve(32),
            requires_another_pass: true,
            table: self.clone(),
        })
    }
}

impl LayoutableTable for Post {
    fn tag(&self) -> [u8; 4] {
        *b"post"
    }
}

struct LayoutedPost {
    reservation: Reservation,
    requires_another_pass: bool,
    table: Post,
}

impl Layouted for LayoutedPost {
    fn reservation(&self) -> &Reservation {
        &self.reservation
    }

    fn requires_another_pass(&self) -> bool {
        self.requires_another_pass
    }

    fn pass(&mut self, _current_file: &[u8]) -> Result<(), crate::LayoutError> {
        use crate::open_type::FixedWriteExt;
        use byteorder::{WriteBytesExt, BE};

        self.requires_another_pass = false;

        let mut writer = self.reservation.writer();

        writer.write_u16::<BE>(3)?; // Version major
        writer.write_u16::<BE>(0)?; // Version minor

        writer.write_fixed::<BE>(&self.table.italic_angle)?;

        writer.write_i16::<BE>(self.table.underline_position)?;
        writer.write_i16::<BE>(self.table.underline_thickness)?;

        writer.write_u32::<BE>(if self.table.is_fixed_pitch { 1 } else { 0 })?;

        writer.write_u32::<BE>(self.table.min_mem_type42)?;
        writer.write_u32::<BE>(self.table.max_mem_type42)?;

        writer.write_u32::<BE>(self.table.min_mem_type1)?;
        writer.write_u32::<BE>(self.table.max_mem_type1)?;

        Ok(())
    }
}

impl LayoutedTable for LayoutedPost {
    fn tag(&self) -> [u8; 4] {
        self.table.tag()
    }
}
