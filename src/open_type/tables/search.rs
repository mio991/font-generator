pub struct SearchData {
    pub search_range: u16,
    pub entry_selector: u16,
    pub range_shift: u16,
}

impl SearchData {
    pub fn for_length(lenght: u16) -> Self {
        let entry_selector = lenght.ilog2() as u16;
        let search_range = u16::pow(2, entry_selector as u32) * 16;
        let range_shift = lenght * 16 - search_range;

        Self {
            search_range,
            entry_selector,
            range_shift,
        }
    }
}
