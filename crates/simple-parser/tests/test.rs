#[cfg(test)]
mod test {
    use simple_parser::Parse;
    use simple_parser::Result;
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
}
