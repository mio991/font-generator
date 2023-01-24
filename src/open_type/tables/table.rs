/*
impl dyn Table {
    pub fn store(&self, writer: &mut dyn Write, offset: u32) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();

        self.store_internal(&mut buffer)?;

        writer.write_all(&self.get_tag())?;
        writer.write_u32::<BE>(0)?;
        writer.write_u32::<BE>(offset)?;
        writer.write_u32::<BE>(buffer.len() as u32)?;

        return Ok(buffer);
    }
}
*/
