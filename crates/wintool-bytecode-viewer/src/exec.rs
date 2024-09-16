use garygo_jass_common::show_bytecode_lines;
use std::env;
use std::path::Path;

mod hack;
mod process;

fn parse_u32(s: &str) -> Result<u32, std::num::ParseIntError> {
    match s {
        s if s.starts_with("0x") => u32::from_str_radix(&s[2..], 16),
        _ => s.parse(),
    }
}

pub fn exec() {
    let mut args = env::args();

    let path = args.next().unwrap();
    let name = Path::new(&path)
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();
    let address = args.next().map(|t| parse_u32(t.as_str()).ok()).flatten();
    let len = args
        .next()
        .map(|t| t.parse::<usize>().ok())
        .flatten()
        .unwrap_or(100);

    if let Some(address) = address {
        println!("check {address} {len}");
        let r = hack::hack(address, len);
        show_bytecode_lines(&r[..]);
    } else {
        println!("invaild param, correct example: '{name} 366949968 10'")
    }
}