mod bytecode;
mod bytecode_viewer;

pub use bytecode::Bytecode;
pub use bytecode::BytecodeValueType;
pub use bytecode::Reg;
pub use bytecode::SymbolId;
pub use bytecode_viewer::show_bytecode_lines;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
