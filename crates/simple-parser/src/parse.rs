use crate::Lex;
use crate::Result;
use crate::Token;
use garygo_jass_common::Bytecode;
use garygo_jass_common::BytecodeValueType;
use garygo_jass_common::Reg;
use garygo_jass_common::SymbolId;
use std::collections::HashMap;
use std::fmt::Debug;
use std::io::Read;

#[derive(Clone, PartialEq)]
pub struct ScriptType {
    name: String,
    extends: String,
    base: BytecodeValueType,
}

impl Debug for ScriptType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.extends.len() > 0 {
            write!(
                f,
                "ScriptType({} extends {})",
                self.name.as_str(),
                self.extends.as_str()
            )
        } else {
            write!(f, "ScriptType({})", self.name.as_str())
        }
    }
}

#[derive(Clone)]
pub struct FunctionArg {
    name: String,
    script_type: ScriptType,
    #[allow(dead_code)]
    idx: u8,
}

#[derive(Clone)]
pub struct Function {
    name: String,
    args: Vec<FunctionArg>,
    ret: Option<ScriptType>,
}

impl Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "function {}(", self.name)?;
        for FunctionArg {
            name, script_type, ..
        } in &self.args
        {
            write!(f, " {} {name}", script_type.name)?;
        }
        write!(f, ") => {:?}", self.ret)?;
        Ok(())
    }
}

pub struct Parse<R: Read> {
    bytecodes: Vec<Bytecode>,
    symbol_table: Vec<String>,
    types: HashMap<usize, ScriptType>,
    functions: HashMap<usize, Function>,
    natives: HashMap<usize, Function>,
    var_type: HashMap<usize, ScriptType>,
    lex: Lex<R>,
    reg: u8,
}

impl<R: Read> Parse<R> {
    fn new(r: R) -> Parse<R> {
        Parse {
            bytecodes: vec![],
            lex: Lex::new(r),
            symbol_table: vec![],
            types: HashMap::new(),
            functions: HashMap::new(),
            natives: HashMap::new(),
            var_type: HashMap::new(),
            reg: 0x00,
        }
    }

    fn peek(&mut self) -> Result<&Token> {
        self.lex.peek()
    }

    fn next(&mut self) -> Result<Token> {
        let peek = self.lex.peek()?;
        println!("{:?}", peek.clone());
        self.lex.next()
    }

    fn guess(&mut self, guess: &Token) -> Result<bool> {
        let token = self.lex.peek()?;
        Ok(token == guess)
    }

    fn expect_consume(&mut self, expect: &Token) -> Result<()> {
        if !self.guess(expect)? {
            return Err(format!("unexpect token: {expect:?}").into());
        }
        self.next()?;
        Ok(())
    }

    fn symbol_index(&mut self, symbol: &str) -> Result<usize> {
        let table = &mut self.symbol_table;
        let position = table
            .iter()
            .position(|v| v.as_str() == symbol)
            .unwrap_or_else(|| {
                table.push(symbol.to_string());
                table.len() - 1
            });
        Ok(position)
    }

    fn typeinfo(&mut self, symbol: &str) -> Result<ScriptType> {
        let idx = self.symbol_index(symbol)?;
        let script_type = match self.types.get(&idx).cloned() {
            Some(types) => types,
            None => todo!(),
        };

        Ok(script_type)
    }

    /// 依赖文法保证安全性
    fn set_var_type(&mut self, symbol: SymbolId, script_type: ScriptType) {
        let idx = u32::from(symbol) as usize;
        self.var_type.insert(idx, script_type);
    }

    fn get_var_type(&self, symbol: SymbolId) -> Result<&ScriptType> {
        let idx = u32::from(symbol) as usize;
        let t = self.var_type.get(&idx);
        match t {
            Some(x) => Ok(x),
            None => Err("not found var defined".into()),
        }
    }

    fn next_reg(&mut self) -> u8 {
        if self.reg == u8::MAX {
            self.reg = 1
        } else {
            self.reg += 1;
        }
        self.reg
    }
}

#[derive(Clone)]
struct Exp {
    exp_type: ScriptType,
    pos: u8,
    #[allow(dead_code)]
    priority: usize,
}

#[rustfmt::skip]
impl Token {
    fn is_binop(&self) -> bool {
        match &self {
            Token::And | Token::Or | Token::Add | Token::Sub | Token::Mul
            | Token::Div | Token::Equal | Token::NotEq | Token::LesEq
            | Token::GreEq | Token::Less | Token::Greater
              => true,
            _ => false,
        }
    }

    fn binop_bytecode(&self) -> fn(Reg, Reg, Reg) -> Bytecode {
        match &self {
            Token::And => Bytecode::And,
            Token::Or => Bytecode::Or,
            Token::Equal => Bytecode::Equal,
            Token::NotEq => Bytecode::Notequal,
            Token::LesEq => Bytecode::Lesserequal,
            Token::GreEq => Bytecode::Greaterequal,
            Token::Less => Bytecode::Lesser,
            Token::Greater => Bytecode::Greater,
            Token::Add => Bytecode::Add,
            Token::Sub => Bytecode::Sub,
            Token::Mul => Bytecode::Mul,
            Token::Div => Bytecode::Div,
            _ => panic!("invail binop")
        }
    }

    fn priority(&self) -> isize {
        match self {
            Token::Add => 1,
            Token::Sub => 1,
            Token::Div => 2,
            Token::Mul => 2,
            _ => -1,
        }
    }

}

impl<R: Read> Parse<R> {
    fn do_binop(
        &mut self,
        binop: fn(Reg, Reg, Reg) -> Bytecode,
        left: Exp,
        right: Exp,
    ) -> Result<Exp> {
        let reg = self.next_reg();
        self.bytecodes
            .push(binop(reg.into(), left.pos.into(), right.pos.into()));
        return Ok(Exp {
            exp_type: left.exp_type.clone(),
            pos: reg,
            priority: 0,
        });
    }

    fn binop(&mut self, binop: Token, left: Exp, right: Exp) -> Result<Exp> {
        let op = binop.binop_bytecode();
        let left_type_name = left.exp_type.name.clone();
        let right_type_name = right.exp_type.name.clone();

        if left_type_name == right_type_name
            && left_type_name == "string"
            && !matches!(binop, Token::Add)
        {
            return self.do_binop(op, left, right);
        }

        if (left_type_name == "integer" || left_type_name == "real")
            && (right_type_name == "integer" || right_type_name == "real")
        {
            if left_type_name == right_type_name {
                // is ok
            } else if left_type_name == "integer" {
                self.bytecodes.push(Bytecode::IntToReal(left.pos.into()));
            } else {
                assert_eq!(right_type_name, "integer");
                self.bytecodes.push(Bytecode::IntToReal(right.pos.into()));
            }

            return self.do_binop(op, left, right);
        }

        Err("invail binop".into())
    }

    ///! exp ::= name | int | float | exp + exp | exp - exp | exp * exp | exp / exp| funcall | ( exp ) | name[exp]
    ///! funcall ::= name ( explist )

    ///! exp ::= (name | int | float) beta
    ///!beta ::= (+ exp| - exp | * exp | / exp | ( explist) )
    fn expression(&mut self, op_priority: isize) -> Result<Exp> {
        let token = self.next()?;
        let left = match token {
            Token::Null => {
                let reg = self.next_reg();
                self.bytecodes.push(Bytecode::SetRegLiteral(
                    reg.into(),
                    BytecodeValueType::Null,
                    0,
                ));
                Exp {
                    exp_type: self.typeinfo("null")?,
                    pos: reg,
                    priority: 0,
                }
            }
            Token::True | Token::False => {
                let reg = self.next_reg();
                self.bytecodes.push(Bytecode::SetRegLiteral(
                    reg.into(),
                    BytecodeValueType::Boolean,
                    if token == Token::True { 1 } else { 0 },
                ));
                Exp {
                    exp_type: self.typeinfo("boolean")?,
                    pos: reg,
                    priority: 0,
                }
            }
            Token::Integer(i) => {
                let reg = self.next_reg();
                self.bytecodes.push(Bytecode::SetRegLiteral(
                    reg.into(),
                    BytecodeValueType::Integer,
                    i as u64 as u32,
                ));
                Exp {
                    exp_type: self.typeinfo("integer")?,
                    pos: reg,
                    priority: 0,
                }
            }
            Token::Float(i) => {
                let reg = self.next_reg();
                self.bytecodes.push(Bytecode::SetRegLiteral(
                    reg.into(),
                    BytecodeValueType::Real,
                    (i as f32).to_bits(),
                ));
                Exp {
                    exp_type: self.typeinfo("real")?,
                    pos: reg,
                    priority: 0,
                }
            }
            Token::ParL => {
                let exp = self.expression(op_priority)?;
                self.expect_consume(&Token::ParR)?;
                exp
            }
            Token::Name(i) => {
                if self.guess(&Token::ParL)? {
                    let func_idx = self.symbol_index(i.as_str())?;
                    let func_ret = self.functioncall(func_idx)?;
                    let ret_type = match func_ret {
                        Some(ret_type) => ret_type,
                        None => return Err("return nothing is not exp".into()),
                    };
                    let reg = self.next_reg();
                    self.bytecodes.push(Bytecode::Move(reg.into(), 0.into()));
                    Exp {
                        exp_type: ret_type,
                        pos: reg,
                        priority: 0,
                    }
                } else {
                    // var
                    let symbol = self.symbol_index(i.as_str())?;
                    let symbol = SymbolId(symbol as u32);
                    let var_type = self.get_var_type(symbol)?.clone();
                    let reg = self.next_reg();
                    self.bytecodes
                        .push(Bytecode::SetRegVar(reg.into(), var_type.base, symbol));

                    Exp {
                        exp_type: var_type,
                        pos: reg,
                        priority: 0,
                    }
                }
            }
            _ => return Err(format!("not support exp: {token:?}").into()),
        };

        let token = self.peek()?.clone();
        if !token.is_binop() {
            return Ok(left);
        };

        let mut left = left;
        loop {
            let token = self.peek()?.clone();
            if !token.is_binop() {
                return Ok(left);
            };

            let candidate_op_priority = token.priority();
            if candidate_op_priority <= op_priority {
                // candidate is next_op_priority
                // (prev_exp ?op_priority cur_exp) ?next_op_priority otherexp
                return Ok(left);
            }

            // candidate is cur_op_priority
            // prev_exp ?op_priority ( cur_exp ?cur_op_priority otherexp )
            let binop = self.next()?;
            let cur_op_priority = candidate_op_priority;
            let right = self.expression(cur_op_priority)?;
            left = self.binop(binop, left, right)?;
        }
    }

    fn next_symbol(&mut self) -> Result<(usize, String)> {
        let token = self.next()?;
        let name = match token {
            Token::Name(name) => name,
            _ => return Err(format!("unexpect symbol: {token:?}").into()),
        };
        let idx = self.symbol_index(name.as_str())?;
        Ok((idx, name))
    }

    fn next_type(&mut self) -> Result<(usize, ScriptType)> {
        let (next_idx, next_name) = self.next_symbol()?;
        let script_type = match self.types.get(&next_idx) {
            Some(script_type) => script_type.clone(),
            None => return Err(format!("not found type: {next_name}").into()),
        };
        Ok((next_idx, script_type))
    }

    fn function_head(&mut self) -> Result<&Function> {
        let token = self.next()?;
        let function_token = match token {
            Token::Function => Token::Function,
            Token::Native => Token::Native,
            _ => return Err(format!("expect function | native, but {token:?}").into()),
        };

        let (func_idx, func_name) = self.next_symbol()?;
        if self.functions.contains_key(&func_idx) || self.natives.contains_key(&func_idx) {
            return Err(format!("duplicate definition function :{func_name}").into());
        }

        let mut func = Function {
            name: func_name,
            args: vec![],
            ret: None,
        };
        self.bytecodes
            .push(Bytecode::Function(SymbolId(func_idx as u32)));

        self.expect_consume(&Token::Takes)?;
        let token = self.peek()?;
        let is_nothing = matches!(token, Token::Nothing);

        if !is_nothing {
            loop {
                let idx = func.args.len() as u8;
                if idx > 0 {
                    let token = self.peek()?;
                    match token {
                        Token::Comma => {
                            self.next()?;
                        }
                        Token::Returns => break,
                        _ => return Err("except ','".into()),
                    }
                }

                let (_, arg_type) = self.next_type()?;
                let (arg_idx, arg_name) = self.next_symbol()?;
                if function_token == Token::Function {
                    self.set_var_type(SymbolId(arg_idx as u32), arg_type.clone());
                    self.bytecodes.push(Bytecode::Funcarg(
                        arg_type.base,
                        idx,
                        SymbolId(arg_idx as u32),
                    ));
                }
                func.args.push(FunctionArg {
                    name: arg_name,
                    script_type: arg_type,
                    idx,
                });
            }
        } else {
            self.expect_consume(&Token::Nothing)?;
        }

        if func.args.len() > 256 {
            return Err("too much param".into());
        }

        self.expect_consume(&Token::Returns)?;
        let token = self.peek()?;
        let is_nothing = matches!(token, Token::Nothing);
        if !is_nothing {
            let (_, ret_type) = self.next_type()?;
            func.ret = Some(ret_type);
        } else {
            self.next()?;
        }

        let f = if function_token == Token::Function {
            self.functions.insert(func_idx, func);
            self.functions.get(&func_idx).unwrap()
        } else {
            assert!(function_token == Token::Native);
            self.natives.insert(func_idx, func);
            self.natives.get(&func_idx).unwrap()
        };

        Ok(f)
    }

    fn set_statment(&mut self) -> Result<()> {
        self.expect_consume(&Token::Set)?;

        // var
        let (var_index, _) = self.next_symbol()?;
        let token = self.peek()?;
        let array_index = if token == &Token::SqurL {
            self.next()?;
            let exp = self.expression(0)?;
            self.expect_consume(&Token::SqurR)?;
            Some(exp.pos)
        } else {
            None
        };

        // =
        self.expect_consume(&Token::Assign)?;

        // var
        let exp = self.expression(0)?;
        if let Some(i) = array_index {
            self.bytecodes.push(Bytecode::SetVarArray(
                i.into(),
                exp.pos.into(),
                SymbolId(var_index as u32),
            ));
        } else {
            self.bytecodes
                .push(Bytecode::SetVar(exp.pos.into(), SymbolId(var_index as u32)));
        }

        Ok(())
    }

    fn functioncall(&mut self, func_idx: usize) -> Result<Option<ScriptType>> {
        let (op, func) =
            if self.functions.contains_key(&func_idx) || self.natives.contains_key(&func_idx) {
                let function = self.functions.get(&func_idx);
                let native = self.natives.get(&func_idx);
                let is_func = function.is_some();
                let call = if function.is_some() {
                    Bytecode::Calljass
                } else {
                    Bytecode::Callnative
                };
                match if is_func { function } else { native } {
                    Some(func) => (call, func),
                    None => return Err("".into()),
                }
            } else {
                return Err("not found function".into());
            };

        let func = func.clone();
        let param_amount = func.args.len();
        let func_ret = func.ret.clone();
        let mut param = 0;
        self.expect_consume(&Token::ParL)?;
        while param < param_amount {
            if param > 0 {
                let token = self.peek()?;
                match token {
                    Token::ParR => break,
                    Token::Comma => {
                        self.next()?;
                        {}
                    }
                    _ => return Err(format!("expect ',', but {token:?}").into()),
                }
            }

            param += 1;
            let exp = self.expression(0)?;
            let arg = match func.args.get(param - 1) {
                Some(arg) => arg,
                None => return Err("function call params amount is incorrect!".into()),
            };
            if exp.exp_type != arg.script_type {
                return Err("function call params type is not same".into());
            }
            self.bytecodes.push(Bytecode::Push(exp.pos.into()));
        }
        self.expect_consume(&Token::ParR)?;
        if param != param_amount {
            return Err("function call params amount is incorrect!".into());
        }
        self.bytecodes.push(op(SymbolId(func_idx as u32)));
        self.bytecodes.push(Bytecode::Popn(param_amount as u8));
        Ok(func_ret)
    }

    fn functioncall_statment(&mut self) -> Result<()> {
        self.expect_consume(&Token::Call)?;
        let (func_idx, _) = self.next_symbol()?;
        self.functioncall(func_idx)?;
        Ok(())
    }

    ///! var_declared ::= [constant | local] type name = exp
    fn var_declared(&mut self) -> Result<()> {
        let op = match self.peek()? {
            Token::Constant => Bytecode::Constant,
            Token::Local => Bytecode::Local,
            _ => Bytecode::Global,
        };

        if op != Bytecode::Global {
            self.next()?;
        }

        // type
        let (type_index, type_name) = self.next_symbol()?;
        let script_type = match self.types.get(&type_index) {
            Some(st) => st.clone(),
            None => return Err(format!("not found type: {type_name}").into()),
        };

        // var
        let var_index = self.next_symbol()?.0;
        let bytecode = op(script_type.base, SymbolId(var_index as u32));
        self.bytecodes.push(bytecode);
        self.set_var_type(SymbolId(var_index as u32), script_type);

        if !self.guess(&Token::Assign)? {
            // 只定义变量不赋值
            return Ok(());
        } else {
            self.next()?;
        }

        let exp = self.expression(0)?;
        self.bytecodes
            .push(Bytecode::SetVar(exp.pos.into(), SymbolId(var_index as u32)));

        Ok(())
    }

    ///! global_variables ::= global {var_declared} endglobal
    fn global_variables(&mut self) -> Result<()> {
        self.expect_consume(&Token::Globals)?; //
        loop {
            let token = self.peek()?;
            match token {
                Token::Endglobals => {
                    self.next()?;
                    break;
                }
                _ => self.var_declared()?,
            }
        }
        Ok(())
    }

    fn type_definition(&mut self) -> Result<()> {
        self.next()?; // type
        let derived = self.next_symbol()?; // name
        self.expect_consume(&Token::Extends)?; //extends
        let (base_index, base_name) = self.next_symbol()?; // name

        let base = match self.types.get(&base_index) {
            Some(st) => st.clone(),
            None => return Err(format!("not found type: {base_index} {base_name}").into()),
        };
        self.types.insert(
            derived.0,
            ScriptType {
                name: derived.1.clone(),
                extends: base.name.clone(),
                base: base.base.clone(),
            },
        );
        self.bytecodes.push(Bytecode::Type(derived.0.into()));
        self.bytecodes.push(Bytecode::Extends(base_index.into()));
        Ok(())
    }

    fn native_function(&mut self) -> Result<()> {
        let constants = self.guess(&Token::Constant)?;
        if constants {
            self.next()?;
        }
        self.function_head()?;
        Ok(())
    }

    ///! udf ::= function name takes type name {, type name} returns type { var_declared } { stat } [return] endfunction
    fn user_defined_function(&mut self) -> Result<()> {
        let ret = self.function_head()?.ret.is_some();
        loop {
            let token = self.peek()?;
            match token {
                Token::Local => {
                    self.next()?;
                    self.var_declared()?;
                }
                _ => {
                    break;
                }
            }
        }

        loop {
            let token = self.peek()?;
            match token {
                Token::Set => {
                    self.set_statment()?;
                }
                Token::Call => {
                    self.functioncall_statment()?;
                }
                Token::Return => {
                    self.next()?;
                    self.bytecodes.push(Bytecode::Return);
                    if ret {
                        self.expression(0)?;
                    }
                }
                Token::Endfunction => {
                    self.next()?;
                    return Ok(());
                }
                _ => return Err(format!("invail token: {token:?}").into()),
            }
        }
    }

    ///! BNF
    ///! file ::= {global declarations}
    ///! global declarations ::= global_variables | type_definition | native_function | user_defined_function
    pub fn file(&mut self) -> Result<()> {
        loop {
            let token = self.peek()?;
            match token {
                Token::Globals => self.global_variables()?,
                Token::Type => self.type_definition()?,
                Token::Native | Token::Constant => self.native_function()?,
                Token::Function => self.user_defined_function()?,
                Token::Eos => break,
                _ => return Err(format!("unexpect statment {token:?}").into()),
            }
        }
        Ok(())
    }
}

impl<R: Read> Parse<R> {
    fn with_basetype(mut self, symbol: &str, base: BytecodeValueType) -> Result<Parse<R>> {
        let symbol_idx = self.symbol_index(symbol)?;
        self.types.insert(
            symbol_idx,
            ScriptType {
                name: symbol.to_string(),
                extends: "".to_string(),
                base,
            },
        );
        Ok(self)
    }

    pub fn test_instance(r: R) -> Result<Parse<R>> {
        let new = Parse::new(r)
            .with_basetype("code", BytecodeValueType::Code)?
            .with_basetype("integer", BytecodeValueType::Integer)?
            .with_basetype("real", BytecodeValueType::Real)?
            .with_basetype("string", BytecodeValueType::String)?
            .with_basetype("handle", BytecodeValueType::Handle)?
            .with_basetype("boolean", BytecodeValueType::Boolean)?
            .with_basetype("integer", BytecodeValueType::Integer)?
            .with_basetype("null", BytecodeValueType::Null)?;
        Ok(new)
    }

    pub fn show(&self) {
        let bytecodes = &self.bytecodes;
        let symbol_table = &self.symbol_table;
        let types = &self.types;
        let functions = &self.functions;
        println!("symbol_table:");
        for (idx, symbol) in symbol_table.iter().enumerate() {
            let t = types.get(&idx).cloned();
            let f = functions.get(&idx).cloned();

            let t = t
                .and_then(|t| Some(format!(" {t:?}")))
                .unwrap_or("".to_string());
            let f = f
                .and_then(|t| Some(format!(" {t:?}")))
                .unwrap_or("".to_string());
            println!("{idx}:{symbol}{t}{f}")
        }
        println!("");
        println!("");
        println!("bytecode:");
        for (idx, bytecode) in bytecodes.iter().enumerate() {
            println!("{idx}:{bytecode:?}")
        }
    }
}

#[test]
fn test_type_def() -> Result<()> {
    use std::io::Cursor;

    let input_str = "type agent			    extends     handle \n type event			    extends     agent";
    let mut parse = Parse::test_instance(Cursor::new(input_str))?;
    parse.file()?;
    parse.show();

    Ok(())
}

#[test]
fn test_native_func_def() -> Result<()> {
    use std::io::Cursor;

    let input_str = "constant native GetObjectName               takes integer objectId          returns string\nnative GetObjectName2               takes integer objectId          returns string";
    let mut parse = Parse::test_instance(Cursor::new(input_str))?;
    parse.file()?;
    parse.show();

    Ok(())
}

#[test]
fn test_var_def() -> Result<()> {
    use std::io::Cursor;

    let input_str =
        "globals \n constant integer b = 20 \n constant integer a = 5 + 10 * b \n endglobals";
    let mut parse = Parse::test_instance(Cursor::new(input_str))?;
    parse.file()?;
    parse.show();

    Ok(())
}
