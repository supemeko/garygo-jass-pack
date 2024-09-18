use crate::Result;
use std::{
    io::{Bytes, Read},
    iter::Peekable,
    mem,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // keyword
    Globals,
    Endglobals,
    Constant,
    Native,
    Array,
    And,
    Or,
    Not,
    Type,
    Extends,
    Function,
    Endfunction,
    Nothing,
    Takes,
    Returns,
    Call,
    Set,
    Return,
    If,
    Then,
    Endif,
    Elseif,
    Else,
    Loop,
    Endloop,
    Exitwhen,
    Local,
    True,
    False,
    Null,

    // arithmetic sign
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    NotEq,
    LesEq,
    GreEq,
    Less,
    Greater,

    // logical symbol
    Assign,
    ParL,
    ParR,
    SqurL,
    SqurR,
    Comma,

    Integer(i64),
    Float(f64),
    String(Vec<u8>),

    Name(String),

    Eos,
}

impl Eq for Token {}

pub struct Lex<R: Read> {
    input: Peekable<Bytes<R>>,
    ahead: Token,
}

impl<R: Read> Lex<R> {
    pub fn new(input: R) -> Self {
        Lex {
            input: input.bytes().peekable(),
            ahead: Token::Eos,
        }
    }

    pub fn peek(&mut self) -> Result<&Token> {
        if self.ahead == Token::Eos {
            self.ahead = self.next_token()?;
        }
        Ok(&self.ahead)
    }

    pub fn next(&mut self) -> Result<Token> {
        let f = if self.ahead == Token::Eos {
            self.next_token()?
        } else {
            mem::replace(&mut self.ahead, Token::Eos)
        };
        Ok(f)
    }

    fn next_byte(&mut self) -> Result<Option<u8>> {
        let res = self.input.next().transpose()?;
        Ok(res)
    }

    fn peek_byte(&mut self) -> Result<Option<u8>> {
        match self.input.peek() {
            Some(Ok(byte)) => Ok(Some(*byte)),
            Some(Err(e)) => Err(e.to_string().into()),
            None => Ok(None),
        }
    }

    fn guess_byte(&mut self, ch: u8) -> Result<bool> {
        let byte = self.peek_byte()?;
        Ok(byte == Some(ch))
    }

    pub fn expect(&mut self, ch: u8) -> Result<()> {
        if !self.guess_byte(ch)? {
            return Err(format!("expect {}", ch as char).into());
        }
        Ok(())
    }
}

impl<R: Read> Lex<R> {
    ///! 读取字符串类型字面量
    fn read_str(&mut self, first: u8) -> Result<Token> {
        assert_eq!(first as char, '\"');
        let mut str = Vec::new();
        loop {
            let ch = match self.next_byte()? {
                Some(ch) => ch,
                None => return Err("expect(\")".into()),
            };
            match ch {
                b'\"' => break,
                _ => str.push(ch),
            }
        }
        Ok(Token::String(str))
    }

    ///! 读取数字，整数或浮点数
    fn read_number(&mut self, first: u8) -> Result<Token> {
        let mut str = String::new();
        str.push(first as char);
        let mut dot = false;
        loop {
            let ch = match self.peek_byte()? {
                Some(ch) => ch,
                None => {
                    break;
                }
            };
            match ch {
                b'0'..=b'9' => {
                    self.next_byte()?;
                    str.push(ch as char);
                }
                b'.' => {
                    if dot {
                        break;
                    }
                    dot = true;
                    self.next_byte()?;
                    str.push(ch as char);
                }
                _ => break,
            }
        }

        return Ok(if dot {
            Token::Float(str.parse::<f64>()?)
        } else {
            Token::Integer(str.parse::<i64>()?)
        });
    }

    ///! 读取特殊数字，单引号包裹的4位id
    fn read_snumber(&mut self, first: u8) -> Result<Token> {
        assert_eq!(first as char, '\'');
        let mut str = Vec::new();
        loop {
            let ch = match self.next_byte()? {
                Some(ch) => ch,
                None => return Err("expect(\')".into()),
            };
            match ch {
                b'\'' => break,
                _ => str.push(ch),
            }
        }
        if str.len() != 4 {
            println!("error str: {str:?}");
            return Err("invaild single quotes number".into());
        }
        let p1 = *str.get(0).unwrap() as i64 * 255 * 255 * 255;
        let p2 = *str.get(1).unwrap() as i64 * 255 * 255;
        let p3 = *str.get(2).unwrap() as i64 * 255;
        let p4 = *str.get(3).unwrap() as i64;
        Ok(Token::Integer(p1 + p2 + p3 + p4))
    }

    fn skip_annotations(&mut self) -> Result<()> {
        let ch = self.next_byte()?.unwrap();
        assert!(ch == b'*' || ch == b'/');
        if ch == b'/' {
            loop {
                let ch = match self.next_byte()? {
                    Some(ch) => ch,
                    None => break,
                };

                match ch {
                    b'\n' | b'\r' => break,
                    _ => {}
                }
            }
        } else {
            let mut star = false;
            loop {
                let ch = match self.next_byte()? {
                    Some(ch) => ch,
                    None => break,
                };

                match ch {
                    b'*' => {
                        star = true;
                        continue;
                    }
                    b'/' if star => {
                        break;
                    }
                    _ => {}
                }
                star = false;
            }
        }

        Ok(())
    }

    ///! 读取一个名字，名字可能是关键字也可能是变量名称
    fn read_name(&mut self, first: u8) -> Result<Token> {
        let mut name = String::new();
        name.push(first as char);
        loop {
            let ch = match self.peek_byte()? {
                Some(ch) => ch,
                None => break,
            };

            match ch {
                b' ' | b'\n' | b'\r' | b'\t' | b',' | b'(' | b')' | b'=' => break,
                _ => {
                    self.next_byte()?;
                    name.push(ch as char);
                }
            }
        }
        let token = match name.as_str() {
            "globals" => Token::Globals,
            "endglobals" => Token::Endglobals,
            "constant" => Token::Constant,
            "native" => Token::Native,
            "array" => Token::Array,
            "and" => Token::And,
            "or" => Token::Or,
            "not" => Token::Not,
            "type" => Token::Type,
            "extends" => Token::Extends,
            "function" => Token::Function,
            "endfunction" => Token::Endfunction,
            "nothing" => Token::Nothing,
            "takes" => Token::Takes,
            "returns" => Token::Returns,
            "call" => Token::Call,
            "set" => Token::Set,
            "return" => Token::Return,
            "if" => Token::If,
            "then" => Token::Then,
            "endif" => Token::Endif,
            "elseif" => Token::Elseif,
            "else" => Token::Else,
            "loop" => Token::Loop,
            "endloop" => Token::Endloop,
            "exitwhen" => Token::Exitwhen,
            "local" => Token::Local,
            "true" => Token::True,
            "false" => Token::False,
            "null" => Token::Null,
            _ => Token::Name(name),
        };
        Ok(token)
    }

    ///! 读取一个token
    fn next_token(&mut self) -> Result<Token> {
        let ch = match self.input.next().transpose()? {
            Some(ch) => ch,
            None => return Ok(Token::Eos),
        };
        let token = match ch {
            b' ' | b'\n' | b'\r' | b'\t' => self.next_token()?,
            b'0'..=b'9' => self.read_number(ch)?,
            b'a'..=b'z' | b'_' | b'A'..=b'Z' => self.read_name(ch)?,
            b'\'' => self.read_snumber(ch)?,
            b'\"' => self.read_str(ch)?,
            b'+' => Token::Add,
            b'-' => Token::Sub,
            b'*' => Token::Mul,
            b'/' => {
                if self.guess_byte(b'/')? || self.guess_byte(b'*')? {
                    self.skip_annotations()?;
                    self.next()?
                } else {
                    Token::Div
                }
            }
            b'=' => {
                if self.guess_byte(b'=')? {
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            b'!' => {
                if self.guess_byte(b'=')? {
                    Token::NotEq
                } else {
                    Token::Not
                }
            }
            b'<' => {
                if self.guess_byte(b'=')? {
                    Token::LesEq
                } else {
                    Token::Less
                }
            }
            b'>' => {
                if self.guess_byte(b'=')? {
                    Token::GreEq
                } else {
                    Token::Greater
                }
            }
            b',' => Token::Comma,
            b'(' => Token::ParL,
            b')' => Token::ParR,
            _ => return Err(format!("invaild char {}", ch as char).into()),
        };
        Ok(token)
    }
}

#[test]
fn test_simple_token() -> Result<()> {
    use std::io::Cursor;

    let n = "world hello 3.14 4444 function SetUnitName";
    let mut lex = Lex::new(Cursor::new(n));
    assert_eq!(lex.next()?, Token::Name("world".to_string()));
    assert_eq!(lex.next()?, Token::Name("hello".to_string()));
    assert_eq!(lex.next()?, Token::Float(3.14));
    assert_eq!(lex.next()?, Token::Integer(4444));
    assert_eq!(lex.next()?, Token::Function);
    assert_eq!(lex.next()?, Token::Name("SetUnitName".to_string()));
    Ok(())
}

#[test]
fn test_single_quotes_number() -> Result<()> {
    use std::io::Cursor;

    let n = "'1234' '5678'";
    let mut lex = Lex::new(Cursor::new(n));
    assert_eq!(lex.next()?, Token::Integer(815751682));
    assert_eq!(lex.next()?, Token::Integer(882338306));
    Ok(())
}

#[test]
fn test_fn() -> Result<()> {
    use std::io::Cursor;

    let n = "function SetUnitName takes nothing returns nothing \n endfunction";
    let mut lex = Lex::new(Cursor::new(n));
    assert_eq!(lex.next()?, Token::Function);
    assert_eq!(lex.next()?, Token::Name("SetUnitName".to_string()));
    assert_eq!(lex.next()?, Token::Takes);
    assert_eq!(lex.next()?, Token::Nothing);
    assert_eq!(lex.next()?, Token::Returns);
    assert_eq!(lex.next()?, Token::Nothing);
    assert_eq!(lex.next()?, Token::Endfunction);
    Ok(())
}

#[test]
fn test_multi_lines_annotation() -> Result<()> {
    use std::io::Cursor;

    let n = "/*function SetUnitName takes nothing returns nothing \n endfunction*/ ";
    let mut lex = Lex::new(Cursor::new(n));
    assert_eq!(lex.next()?, Token::Eos);
    Ok(())
}

#[test]
fn test_line_annotation() -> Result<()> {
    use std::io::Cursor;

    let n = "//function SetUnitName takes nothing returns nothing\nfunction SetUnitName takes nothing returns nothing \n endfunction ";
    let mut lex = Lex::new(Cursor::new(n));
    assert_eq!(lex.next()?, Token::Function);
    assert_eq!(lex.next()?, Token::Name("SetUnitName".to_string()));
    assert_eq!(lex.next()?, Token::Takes);
    assert_eq!(lex.next()?, Token::Nothing);
    assert_eq!(lex.next()?, Token::Returns);
    assert_eq!(lex.next()?, Token::Nothing);
    assert_eq!(lex.next()?, Token::Endfunction);
    Ok(())
}

#[test]
fn test_str() -> Result<()> {
    use std::io::Cursor;

    let n = "\"12345\" \"\"";
    let mut lex = Lex::new(Cursor::new(n));
    assert_eq!(lex.next()?, Token::String("12345".as_bytes().to_vec()));
    assert_eq!(lex.next()?, Token::String(vec![]));
    Ok(())
}

#[test]
fn test_number_and_char() -> Result<()> {
    use std::io::Cursor;

    let n = "constant integer abcdefghijklmnopqrstuvwxyz = 1234567890";
    let mut lex = Lex::new(Cursor::new(n));
    assert_eq!(lex.next()?, Token::Constant);
    assert_eq!(lex.next()?, Token::Name("integer".into()));
    assert_eq!(
        lex.next()?,
        Token::Name("abcdefghijklmnopqrstuvwxyz".into())
    );
    assert_eq!(lex.next()?, Token::Assign);
    assert_eq!(lex.next()?, Token::Integer(1234567890));
    assert_eq!(lex.next()?, Token::Eos);
    Ok(())
}
