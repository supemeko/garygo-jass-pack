#[cfg(test)]
mod test {
    use simple_parser::{Parse, Result};
    use std::io::Cursor;

    #[test]
    fn test_function() -> Result<()> {
        let input_str = include_str!("function.j");
        let mut parse = Parse::test_instance(Cursor::new(input_str))?;
        parse.file()?;
        parse.show();
        Ok(())
    }

    #[test]
    fn test_commonj() -> Result<()> {
        let input_str = include_str!("common.j");
        let mut parse = Parse::test_instance(Cursor::new(input_str))?;
        parse.file()?;
        parse.show();
        Ok(())
    }

    #[test]
    fn test_blizzardj() -> Result<()> {
        let commonj = include_str!("common.j");
        let blizzardj = include_str!("blizzard.j");
        let input_str = format!("{commonj}\n{blizzardj}");
        let mut parse = Parse::test_instance(Cursor::new(input_str.as_str()))?;
        parse.file()?;
        parse.show();
        Ok(())
    }
}
