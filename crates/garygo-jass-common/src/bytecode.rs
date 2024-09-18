use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub enum BytecodeValueType {
    Nothing = 0,
    Unknown = 1,
    Null = 2,
    Code = 3,
    Integer = 4,
    Real = 5,
    String = 6,
    Handle = 7,
    Boolean = 8,
    IntegerArray = 9,
    RealArray = 10,
    StringArray = 11,
    HandleArray = 12,
    BooleanArray = 13,
}

impl From<BytecodeValueType> for u8 {
    fn from(value: BytecodeValueType) -> Self {
        value as u8
    }
}

impl BytecodeValueType {
    pub fn from_u8(value: u8) -> Option<BytecodeValueType> {
        let jass = match value {
            0 => BytecodeValueType::Nothing,
            1 => BytecodeValueType::Unknown,
            2 => BytecodeValueType::Null,
            3 => BytecodeValueType::Code,
            4 => BytecodeValueType::Integer,
            5 => BytecodeValueType::Real,
            6 => BytecodeValueType::String,
            7 => BytecodeValueType::Handle,
            8 => BytecodeValueType::Boolean,
            9 => BytecodeValueType::IntegerArray,
            10 => BytecodeValueType::RealArray,
            11 => BytecodeValueType::StringArray,
            12 => BytecodeValueType::HandleArray,
            13 => BytecodeValueType::BooleanArray,
            _ => return None,
        };
        return Some(jass);
    }
}

#[derive(Clone, Copy)]
pub struct Reg {
    name: u8,
}

impl From<u8> for Reg {
    fn from(value: u8) -> Self {
        Reg { name: value }
    }
}

impl From<Reg> for u8 {
    fn from(value: Reg) -> Self {
        value.name
    }
}

impl Debug for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "reg({:02X})", &self.name)
    }
}

#[derive(Clone, Copy)]
pub struct SymbolId(pub u32);

impl From<u32> for SymbolId {
    fn from(value: u32) -> Self {
        SymbolId(value)
    }
}

impl From<usize> for SymbolId {
    fn from(value: usize) -> Self {
        SymbolId(value as u32)
    }
}

impl From<SymbolId> for u32 {
    fn from(value: SymbolId) -> u32 {
        value.0
    }
}

impl Debug for SymbolId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "var 0x{:X}", self.0)
    }
}

#[derive(Clone, Copy)]
pub struct FunctionId(u32);

impl From<u32> for FunctionId {
    fn from(value: u32) -> Self {
        FunctionId(value)
    }
}

impl From<FunctionId> for u32 {
    fn from(value: FunctionId) -> u32 {
        value.0
    }
}

impl Debug for FunctionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "function 0x{:X}", self.0)
    }
}

pub enum Bytecode {
    Minlimit(u8, u8, u8, u32),
    Endprogram(u8, u8, u8, u32),
    Oldjump(u32),
    Function(SymbolId),
    Endfunction,
    Local(BytecodeValueType, SymbolId),
    Global(BytecodeValueType, SymbolId),
    Constant(BytecodeValueType, SymbolId),
    Funcarg(BytecodeValueType, u8, SymbolId),
    Extends(SymbolId),
    Type(SymbolId),
    Popn(u8),
    SetRegLiteral(Reg, BytecodeValueType, u32),
    Move(Reg, Reg),
    SetRegVar(Reg, BytecodeValueType, SymbolId),
    SetRegCode(Reg, FunctionId),
    SetRegVarArray(Reg, Reg, BytecodeValueType, SymbolId),
    SetVar(Reg, SymbolId),
    SetVarArray(Reg, Reg, SymbolId),
    Push(Reg),
    Pop(Reg),
    Callnative(SymbolId),
    Calljass(SymbolId),
    IntToReal(Reg),
    And(Reg, Reg, Reg),
    Or(Reg, Reg, Reg),
    Equal(Reg, Reg, Reg),
    Notequal(Reg, Reg, Reg),
    Lesserequal(Reg, Reg, Reg),
    Greaterequal(Reg, Reg, Reg),
    Lesser(Reg, Reg, Reg),
    Greater(Reg, Reg, Reg),
    Add(Reg, Reg, Reg),
    Sub(Reg, Reg, Reg),
    Mul(Reg, Reg, Reg),
    Div(Reg, Reg, Reg),
    Mod(Reg, Reg, Reg),
    Negate(Reg),
    Not(Reg),
    Return,
    Label(u32),
    Jumpiftrue(Reg, u32),
    Jumpiffalse(Reg, u32),
    Jump(u32),
    Maxlimit(u8, u8, u8, u32),
}

impl Debug for Bytecode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Bytecode::Minlimit(r1, r2, r3, arg) => {
                write!(f, "Minlimit({r1:02X}, {r2:02X}, {r3:02X}, {arg:08X})")
            }
            Bytecode::Endprogram(r1, r2, r3, arg) => {
                write!(f, "Endprogram({r1:02X}, {r2:02X}, {r3:02X}, {arg:08X})")
            }
            Bytecode::Oldjump(arg) => {
                write!(f, "Oldjump({:08X})", arg)
            }
            Bytecode::Function(arg) => {
                write!(f, "Function({arg:?})",)
            }
            Bytecode::Endfunction => {
                write!(f, "Endfunction")
            }
            Bytecode::Local(r1, arg) => {
                write!(f, "Local({r1:?}, {arg:?})")
            }
            Bytecode::Global(r1, arg) => {
                write!(f, "Global({r1:?}, {arg:?})")
            }
            Bytecode::Constant(r1, arg) => {
                write!(f, "Constant({r1:?}, {arg:?})")
            }
            Bytecode::Funcarg(r1, r2, arg) => {
                write!(f, "Funcarg({r1:?}, {r2:02X}, {arg:?})")
            }
            Bytecode::Extends(arg) => {
                write!(f, "Extends({arg:?})")
            }
            Bytecode::Type(arg) => {
                write!(f, "Type({arg:?})")
            }
            Bytecode::Popn(r1) => {
                write!(f, "Popn({:02X})", r1)
            }
            Bytecode::SetRegLiteral(r1, r2, arg) => {
                write!(f, "SetRegLiteral({r1:?}, {r2:?}, 0x{arg:08X})")
            }
            Bytecode::Move(r1, r2) => {
                write!(f, "Move({r1:?}, {r2:?})")
            }
            Bytecode::SetRegVar(r1, r2, arg) => {
                write!(f, "SetRegVar({r1:?}, {r2:?}, {arg:?})")
            }
            Bytecode::SetRegCode(r1, arg) => {
                write!(f, "SetRegCode({r1:?}, {arg:?})")
            }
            Bytecode::SetRegVarArray(r1, r2, r3, arg) => {
                write!(f, "SetRegVarArray({r1:?}, idx: {r2:?}, {r3:?}, {arg:?})")
            }
            Bytecode::SetVar(r1, arg) => {
                write!(f, "SetVar({r1:?}, {arg:?})")
            }
            Bytecode::SetVarArray(r1, r2, arg) => {
                write!(f, "SetVarArray(idx: {r1:?}, {r2:?}, {arg:?})")
            }
            // fprintf(f, "push r%02X\n", op->r1);
            Bytecode::Push(r1) => {
                write!(f, "Push({r1:?})")
            }
            Bytecode::Pop(r1) => {
                write!(f, "Pop({r1:?})")
            }
            Bytecode::Callnative(arg) => {
                write!(f, "Callnative({arg:?})")
            }
            Bytecode::Calljass(arg) => {
                write!(f, "Calljass({arg:?})")
            }
            Bytecode::IntToReal(r1) => {
                write!(f, "I2r({r1:?})")
            }
            Bytecode::And(r1, r2, r3) => {
                write!(f, "And({r1:?}, {r2:?}, {r3:?})")
            }
            Bytecode::Or(r1, r2, r3) => {
                write!(f, "Or({r1:?}, {r2:?}, {r3:?})")
            }
            Bytecode::Equal(r1, r2, r3) => {
                write!(f, "Equal({r1:?}, {r2:?}, {r3:?})")
            }
            Bytecode::Notequal(r1, r2, r3) => {
                write!(f, "Notequal({r1:?}, {r2:?}, {r3:?})")
            }
            Bytecode::Lesserequal(r1, r2, r3) => {
                write!(f, "Lesserequal({r1:?}, {r2:?}, {r3:?})")
            }
            Bytecode::Greaterequal(r1, r2, r3) => {
                write!(f, "Greaterequal({r1:?}, {r2:?}, {r3:?})")
            }
            Bytecode::Lesser(r1, r2, r3) => {
                write!(f, "Lesser({r1:?}, {r2:?}, {r3:?})")
            }
            Bytecode::Greater(r1, r2, r3) => {
                write!(f, "Greater({r1:?}, {r2:?}, {r3:?})")
            }
            Bytecode::Add(r1, r2, r3) => {
                write!(f, "Add({r1:?}, {r2:?}, {r3:?})")
            }
            Bytecode::Sub(r1, r2, r3) => {
                write!(f, "Sub({r1:?}, {r2:?}, {r3:?})")
            }
            Bytecode::Mul(r1, r2, r3) => {
                write!(f, "Mul({r1:?}, {r2:?}, {r3:?})")
            }
            Bytecode::Div(r1, r2, r3) => {
                write!(f, "Div({r1:?}, {r2:?}, {r3:?})")
            }
            Bytecode::Mod(r1, r2, r3) => {
                write!(f, "Mod({r1:?}, {r2:?}, {r3:?})")
            }
            Bytecode::Negate(r1) => {
                write!(f, "Negate({r1:?})")
            }
            Bytecode::Not(r1) => {
                write!(f, "Not({r1:?})")
            }
            Bytecode::Return => {
                write!(f, "Return")
            }
            Bytecode::Label(arg) => {
                write!(f, "Label({arg:08X})")
            }
            Bytecode::Jumpiftrue(r1, arg) => {
                write!(f, "Jumpiftrue({r1:?}, {arg:08X})")
            }
            Bytecode::Jumpiffalse(r1, arg) => {
                write!(f, "Jumpiffalse({r1:?}, {arg:08X})")
            }
            Bytecode::Jump(arg) => {
                write!(f, "Jump({arg:08X})")
            }
            Bytecode::Maxlimit(r1, r2, r3, arg) => {
                write!(f, "Maxlimit({r1:02X}, {r2:02X}, {r3:02X}, {arg:08X})")
            }
        }
    }
}

impl Bytecode {
    pub fn from(r3: u8, r2: u8, r1: u8, op: u8, arg: u32) -> Option<Bytecode> {
        let bytecode = match op {
            0x00 => Bytecode::Minlimit(r1, r2, r3, arg),
            0x01 => Bytecode::Endprogram(r1, r2, r3, arg),
            0x02 => Bytecode::Oldjump(arg),
            0x03 => Bytecode::Function(arg.into()),
            0x04 => Bytecode::Endfunction,
            0x05 => Bytecode::Local(BytecodeValueType::from_u8(r1)?, arg.into()),
            0x06 => Bytecode::Global(BytecodeValueType::from_u8(r1)?, arg.into()),
            0x07 => Bytecode::Constant(BytecodeValueType::from_u8(r1)?, arg.into()),
            0x08 => Bytecode::Funcarg(BytecodeValueType::from_u8(r1)?, r2, arg.into()),
            0x09 => Bytecode::Extends(arg.into()),
            0x0A => Bytecode::Type(arg.into()),
            0x0B => Bytecode::Popn(r1),
            0x0C => Bytecode::SetRegLiteral(r1.into(), BytecodeValueType::from_u8(r2)?, arg),
            0x0D => Bytecode::Move(r1.into(), r2.into()),
            0x0E => Bytecode::SetRegVar(r1.into(), BytecodeValueType::from_u8(r2)?, arg.into()),
            0x0F => Bytecode::SetRegCode(r1.into(), arg.into()),
            0x10 => Bytecode::SetRegVarArray(
                r1.into(),
                r2.into(),
                BytecodeValueType::from_u8(r3)?,
                arg.into(),
            ),
            0x11 => Bytecode::SetVar(r1.into(), arg.into()),
            0x12 => Bytecode::SetVarArray(r1.into(), r2.into(), arg.into()),
            0x13 => Bytecode::Push(r1.into()),
            0x14 => Bytecode::Pop(r1.into()),
            0x15 => Bytecode::Callnative(arg.into()),
            0x16 => Bytecode::Calljass(arg.into()),
            0x17 => Bytecode::IntToReal(r1.into()),
            0x18 => Bytecode::And(r1.into(), r2.into(), r3.into()),
            0x19 => Bytecode::Or(r1.into(), r2.into(), r3.into()),
            0x1A => Bytecode::Equal(r1.into(), r2.into(), r3.into()),
            0x1B => Bytecode::Notequal(r1.into(), r2.into(), r3.into()),
            0x1C => Bytecode::Lesserequal(r1.into(), r2.into(), r3.into()),
            0x1D => Bytecode::Greaterequal(r1.into(), r2.into(), r3.into()),
            0x1E => Bytecode::Lesser(r1.into(), r2.into(), r3.into()),
            0x1F => Bytecode::Greater(r1.into(), r2.into(), r3.into()),
            0x20 => Bytecode::Add(r1.into(), r2.into(), r3.into()),
            0x21 => Bytecode::Sub(r1.into(), r2.into(), r3.into()),
            0x22 => Bytecode::Mul(r1.into(), r2.into(), r3.into()),
            0x23 => Bytecode::Div(r1.into(), r2.into(), r3.into()),
            0x24 => Bytecode::Mod(r1.into(), r2.into(), r3.into()),
            0x25 => Bytecode::Negate(r1.into()),
            0x26 => Bytecode::Not(r1.into()),
            0x27 => Bytecode::Return,
            0x28 => Bytecode::Label(arg),
            0x29 => Bytecode::Jumpiftrue(r1.into(), arg),
            0x2A => Bytecode::Jumpiffalse(r1.into(), arg),
            0x2B => Bytecode::Jump(arg),
            0x2C => Bytecode::Maxlimit(r1, r2, r3, arg),
            _ => return None,
        };
        return Some(bytecode);
    }
}
