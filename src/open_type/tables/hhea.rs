use crate::{
    layout::Reservation,
    open_type::{LayoutableTable, LayoutedTable},
    Layoutable, Layouted,
};

/**
NOTE: The ascender, descender and linegap values in this table are Apple specific; see Apple's specification for details regarding Apple platforms. The sTypoAscender, sTypoDescender and sTypoLineGap fields in the OS/2 table are used on the Windows platform, and are recommended for new text-layout implementations. Font developers should evaluate behavior in target applications that may use fields in this table or in the OS/2 table to ensure consistent layout. See the descriptions of the OS/2 fields for additional details.
 */
#[derive(Debug, Clone)]
pub struct HHead {
    /**
    Typographic ascent—see note above.
     */
    pub ascender: i16,
    /**
     Typographic descent—see note above.
    */
    pub descender: i16,
    /**
     Typographic line gap. Negative LineGap values are treated as zero in some legacy platform implementations.
    */
    pub line_gap: i16,
    /**
     Maximum advance width value in 'hmtx' table.
    */
    pub advance_width_max: u16,
    /**
     Minimum left sidebearing value in 'hmtx' table for glyphs with contours (empty glyphs should be ignored).
    */
    pub min_left_side_bearing: i16,
    /**
     Minimum right sidebearing value; calculated as min(aw - (lsb + xMax - xMin)) for glyphs with contours (empty glyphs should be ignored).
    */
    pub min_right_side_bearing: i16,
    /**
     Max(lsb + (xMax - xMin)).
    */
    pub x_max_extent: i16,
    /**
     Used to calculate the slope of the cursor (rise/run); 1 for vertical.
    */
    pub caret_slope_rise: i16,
    /**
     0 for vertical.
    */
    pub caret_slope_run: i16,
    /**
     The amount by which a slanted highlight on a glyph needs to be shifted to produce the best appearance. Set to 0 for non-slanted fonts
    */
    pub caret_offset: i16,
    /*
    0 for current format.
    */
    pub metric_data_format: i16,
    /*
    Number of hMetric entries in 'hmtx' table
    */
    pub number_of_hmetrics: u16,
}

impl Layoutable<Box<dyn LayoutedTable>> for HHead {
    fn layout(&self, layouter: &mut crate::Layouter) -> Box<dyn LayoutedTable> {
        Box::new(LayoutedHHead {
            reservation: layouter.reserve(36),
            requires_another_pass: true,
            table: self.clone(),
        })
    }
}

impl LayoutableTable for HHead {
    fn tag(&self) -> [u8; 4] {
        *b"hhea"
    }
}

struct LayoutedHHead {
    reservation: Reservation,
    requires_another_pass: bool,
    table: HHead,
}

impl Layouted for LayoutedHHead {
    fn reservation(&self) -> &Reservation {
        &self.reservation
    }

    fn requires_another_pass(&self) -> bool {
        self.requires_another_pass
    }

    fn pass(&mut self, _current_file: &[u8]) -> Result<(), crate::LayoutError> {
        use byteorder::{WriteBytesExt, BE};

        self.requires_another_pass = false;

        let mut writer = self.reservation.writer();

        writer.write_u16::<BE>(1)?; // major version
        writer.write_u16::<BE>(0)?; // minor version

        writer.write_i16::<BE>(self.table.ascender)?;
        writer.write_i16::<BE>(self.table.descender)?;

        writer.write_i16::<BE>(self.table.line_gap)?;
        writer.write_u16::<BE>(self.table.advance_width_max)?;

        writer.write_i16::<BE>(self.table.min_left_side_bearing)?;
        writer.write_i16::<BE>(self.table.min_right_side_bearing)?;

        writer.write_i16::<BE>(self.table.x_max_extent)?;

        writer.write_i16::<BE>(self.table.caret_slope_rise)?;
        writer.write_i16::<BE>(self.table.caret_slope_run)?;
        writer.write_i16::<BE>(self.table.caret_offset)?;

        writer.write_i16::<BE>(0)?; // reserved
        writer.write_i16::<BE>(0)?; // reserved
        writer.write_i16::<BE>(0)?; // reserved
        writer.write_i16::<BE>(0)?; // reserved

        writer.write_i16::<BE>(self.table.metric_data_format)?;
        writer.write_u16::<BE>(self.table.number_of_hmetrics)?;

        Ok(())
    }
}

impl LayoutedTable for LayoutedHHead {
    fn tag(&self) -> [u8; 4] {
        *b"hhea"
    }
}
