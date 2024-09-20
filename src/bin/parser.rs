use std::io::Cursor;
use std::path::Path;
use std::{env, fs};

use simple_parser::*;

fn main() -> Result<()> {
    let mut args = env::args();

    let path = args.next().unwrap();
    let _ = Path::new(&path)
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();

    let mut files = vec![];
    loop {
        let next = args.next();
        let Some(next) = next else {
            break;
        };
        files.push(next);
    }

    if files.len() == 0 {
        println!("examples: parser file1.j file2.j ...");
        return Ok(());
    }
    let mut concat: Vec<u8> = vec![];
    for file in files {
        let mut cur = fs::read(file).unwrap();
        concat.append(&mut cur);
        concat.push(b'\n');
    }

    // exec
    let mut parse = Parse::test_instance(Cursor::new(concat))?;
    if let Err(x) = parse.file() {
        parse.show_pos();
        return Err(x);
    }
    parse.show();

    Ok(())
}
