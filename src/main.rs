use std::error::Error;

use font_generator::*;

fn main() -> Result<(), Box<dyn Error>> {
    let doc = File::new_with_tables(vec![Box::new(CMap::new_with_ranges(vec![
        CharacterRange {
            start: 'o' as u16,
            end: 'o' as u16,
            start_id: 0,
        },
    ]))]);

    let mut file = Layouter::new();

    let mut doc = doc.layout(&mut file);

    let mut result = file.get_result();

    while doc.requires_another_pass() {
        doc.pass(result.as_slice())?;
        result = file.get_result();
    }

    dbg!(result);

    Ok(())
}
