use crate::Bytecode;
use std::fmt::Debug;

struct ReadBytecode {
    op: u8,
    r1: u8,
    r2: u8,
    r3: u8,
    arg: u32,
}

impl Debug for ReadBytecode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ReadBytecode {
            op,
            r1,
            r2,
            r3,
            arg,
        } = self;
        write!(f, "{op:02X}{r1:02X}{r2:02X}{r3:02X} {arg:08X}")
    }
}

impl ReadBytecode {
    fn bytecode(&self) -> Option<Bytecode> {
        let &ReadBytecode {
            op,
            r1,
            r2,
            r3,
            arg,
        } = self;
        Bytecode::from(r3, r2, r1, op, arg)
    }

    fn line(iter: &mut std::iter::Peekable<std::slice::Iter<'_, u8>>) -> Option<ReadBytecode> {
        let r3 = *iter.next()?;
        let r2 = *iter.next()?;
        let r1 = *iter.next()?;
        let op = *iter.next()?;
        let arg = u32::from_le_bytes([*iter.next()?, *iter.next()?, *iter.next()?, *iter.next()?]);

        Some(ReadBytecode {
            r1,
            r2,
            r3,
            op,
            arg,
        })
    }

    fn lines(list: &[u8]) -> Vec<ReadBytecode> {
        let mut vec = vec![];

        let mut iter: std::iter::Peekable<std::slice::Iter<'_, u8>> = list.iter().peekable();
        while iter.peek().is_some() {
            let bytecode = ReadBytecode::line(&mut iter);
            match bytecode {
                Some(t) => vec.push(t),
                None => break,
            }
        }

        vec
    }

    fn show_line(&self) -> String {
        let bc = self.bytecode();
        match bc {
            Some(x) => format!("{self:?} | {x:?}"),
            None => format!("{self:?} | invalid"),
        }
    }
}

pub fn show_bytecode_lines(list: &[u8]) {
    let bytecodes = ReadBytecode::lines(list);
    for (line_number, bytecode) in bytecodes.iter().enumerate() {
        println!("{line_number:04}:{}", bytecode.show_line());
    }
}
