use std::cell::LazyCell;

use chrono::{DateTime, Utc};

use crate::{
    layout::{Layoutable, Layouted, Reservation},
    open_type::{Fixed, LayoutableTable, LayoutedTable},
};

#[derive(Debug, Clone, Copy)]
pub struct Flags {
    /**
     * Bit 0: Baseline for font at y=0.
     */
    pub baseline: bool,
    /**
     * Bit 1: Left sidebearing point at x=0 (relevant only for TrueType rasterizers).
     */
    pub sidebearing: bool,
    /**
     * Bit 2: Instructions may depend on point size.
     */
    pub depends_on_pointsize: bool,
    /**
     * Bit 3: Force ppem to integer values for all internal scaler math; may use fractional ppem sizes if this bit is clear. It is strongly recommended that this be set in hinted fonts.
     */
    pub force_ppem: bool,
    /**
     * Bit 4: Instructions may alter advance width (the advance widths might not scale linearly).
     */
    pub dynamic_advance_width: bool,
    /**
     * Bit 11: Font data is “lossless” as a result of having been subjected to optimizing transformation and/or compression (such as e.g. compression mechanisms defined by ISO/IEC 14496-18, MicroType Express, WOFF 2.0 or similar) where the original font functionality and features are retained but the binary compatibility between input and output font files is not guaranteed. As a result of the applied transform, the DSIG table may also be invalidated.
     */
    pub lossless: bool,
    /**
     * Bit 12: Font converted (produce compatible metrics).
     */
    pub converted: bool,
    /**
     * Bit 13: Font optimized for ClearType™. Note, fonts that rely on embedded bitmaps (EBDT) for rendering should not be considered optimized for ClearType, and therefore should keep this bit cleared.
     */
    pub cleartype_optimized: bool,
    /**
     * Bit 14: Last Resort font. If set, indicates that the glyphs encoded in the 'cmap' subtables are simply generic symbolic representations of code point ranges and don’t truly represent support for those code points. If unset, indicates that the glyphs encoded in the 'cmap' subtables represent proper support for those code points.
     */
    pub last_resort: bool,
}

impl Default for Flags {
    fn default() -> Self {
        Self {
            baseline: false,
            sidebearing: false,
            depends_on_pointsize: false,
            force_ppem: true,
            dynamic_advance_width: true,
            lossless: false,
            converted: false,
            cleartype_optimized: false,
            last_resort: false,
        }
    }
}

impl Flags {
    pub fn as_u16(&self) -> u16 {
        (if self.baseline { 2 ^ 0 } else { 0 })
            | (if self.sidebearing { 2 ^ 1 } else { 0 })
            | (if self.depends_on_pointsize { 2 ^ 2 } else { 0 })
            | (if self.force_ppem { 2 ^ 3 } else { 0 })
            | (if self.dynamic_advance_width { 2 ^ 4 } else { 0 })
            | (if self.lossless { 2 ^ 11 } else { 0 })
            | (if self.converted { 2 ^ 12 } else { 0 })
            | (if self.cleartype_optimized { 2 ^ 13 } else { 0 })
            | (if self.last_resort { 2 ^ 14 } else { 0 })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Head {
    pub revision: Fixed,
    pub flags: Flags,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    /**
     * Set to a value from 16 to 16384. Any value in this range is valid. In fonts that have TrueType outlines, a power of 2 is recommended as this allows performance optimizations in some rasterizers.
     */
    pub units_per_em: u16,
    /** Smallest readable size in pixels per em */
    pub smalest_recocnizeable_size: u16,
    /** Minimum x coordinate across all glyph bounding boxes. */
    pub min_x: i16,
    /** Minimum y coordinate across all glyph bounding boxes. */
    pub min_y: i16,
    /** Maximum x coordinate across all glyph bounding boxes. */
    pub max_x: i16,
    /** Maximum y coordinate across all glyph bounding boxes. */
    pub max_y: i16,
}

impl LayoutableTable for Head {
    fn tag(&self) -> [u8; 4] {
        *b"head"
    }
}

impl Layoutable<Box<dyn LayoutedTable>> for Head {
    fn layout(&self, layouter: &mut crate::Layouter) -> Box<dyn LayoutedTable> {
        Box::new(LayoutedHead {
            requires_another_pass: true,
            reservation: layouter.reserve(54),
            checksum: 0,
            revision: self.revision.clone(),
            flags: self.flags.as_u16(),
            created: self.created.clone(),
            modified: self.modified.clone(),
            min_x: self.min_x,
            min_y: self.min_y,
            max_x: self.max_x,
            max_y: self.max_y,
            units_per_em: self.units_per_em,
            smalest_recocnizeable_size: self.smalest_recocnizeable_size,
        })
    }
}

struct LayoutedHead {
    requires_another_pass: bool,
    reservation: Reservation,
    revision: Fixed,
    checksum: u32,
    flags: u16,
    units_per_em: u16,
    created: DateTime<Utc>,
    modified: DateTime<Utc>,
    min_x: i16,
    min_y: i16,
    max_x: i16,
    max_y: i16,
    smalest_recocnizeable_size: u16,
}

const EMPOCH: LazyCell<DateTime<Utc>> = LazyCell::new(|| {
    DateTime::parse_from_rfc3339("1904-01-01T00:00:00Z")
        .expect("it is a constant")
        .with_timezone(&Utc)
});

impl Layouted for LayoutedHead {
    fn reservation(&self) -> &Reservation {
        &self.reservation
    }

    fn requires_another_pass(&self) -> bool {
        self.requires_another_pass
    }

    fn pass(&mut self, current_file: &[u8]) -> Result<(), crate::layout::LayoutError> {
        use crate::open_type::FixedWriteExt;
        use byteorder::{WriteBytesExt, BE};

        let old_checksum = self.checksum;
        self.checksum = checksum(current_file)?;

        self.requires_another_pass = old_checksum != self.checksum;

        let mut writer = self.reservation.writer();

        writer.write_u16::<BE>(1)?; // Major Vaersion
        writer.write_u16::<BE>(0)?; // Minor Vaersion

        writer.write_fixed::<BE>(&self.revision)?;
        if self.requires_another_pass {
            writer.write_u32::<BE>(0)?;
        } else {
            writer.write_u32::<BE>(self.checksum)?;
        }

        writer.write_u32::<BE>(0x5F0F3CF5)?; // magicNumber
        writer.write_u16::<BE>(self.flags)?;

        writer.write_u16::<BE>(self.units_per_em)?;

        writer.write_i64::<BE>((self.created - *EMPOCH).num_seconds())?;
        writer.write_i64::<BE>((self.modified - *EMPOCH).num_seconds())?;

        writer.write_i16::<BE>(self.min_x)?;
        writer.write_i16::<BE>(self.min_y)?;

        writer.write_i16::<BE>(self.max_x)?;
        writer.write_i16::<BE>(self.max_y)?;

        writer.write_u16::<BE>(0)?; // macStyle

        writer.write_u16::<BE>(self.smalest_recocnizeable_size)?;

        writer.write_i16::<BE>(2)?; // fontDirectionHint

        writer.write_i16::<BE>(0)?; // indexToLocFormat
        writer.write_i16::<BE>(0)?; // glyphDataFormat

        Ok(())
    }
}

impl LayoutedTable for LayoutedHead {
    fn tag(&self) -> [u8; 4] {
        *b"head"
    }
}

fn checksum(file: &[u8]) -> std::io::Result<u32> {
    use byteorder::{ReadBytesExt, BE};
    use std::io::{Cursor, Read};

    let mut sum: u32 = 0;
    let mut remaining = file.len();

    let mut reader = Cursor::new(file);

    while remaining >= 4 {
        remaining -= 4;

        sum = sum.wrapping_add(reader.read_u32::<BE>()?);
    }

    if remaining > 0 {
        let mut buffer = Vec::with_capacity(4);
        reader.read_to_end(&mut buffer)?;

        buffer.resize(4, 0);

        sum = sum.wrapping_add(buffer.as_slice().read_u32::<BE>()?);
    }

    Ok(0xB1B0AFBAu32.wrapping_sub(sum))
}
