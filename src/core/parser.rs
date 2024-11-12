use core::panic;
use std::{fmt, fs::File};

use super::{
    lexer::{Token, Lexer},
    vm::ExeState,
};

#[derive(Clone)]
pub enum Value {
    Nil,
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Function(fn(&mut ExeState) -> i32),
}

#[derive(Debug)]
pub enum ByteCode {
    GetGlobal(u8, u8),
    LoadGlobal(u8, u8),
    LoadInt(u8, i16),
    LoadBool(u8, bool),
    LoadNil(u8),
    Call(u8, u8),
    LoadConst(u8, u8)
}

#[derive(Debug)]
pub struct ParseProto {
    pub constants: Vec<Value>,
    pub byte_codes: Vec<ByteCode>,
}

pub fn load(input: File) -> ParseProto {
    let mut constants: Vec<Value> = vec![];
    let mut byte_codes: Vec<ByteCode> = vec![];
    let mut lexer = Lexer::new(input);
    loop {
        match lexer.next() {
            Token::Name(name) => {
                constants.push(Value::String(name));
                byte_codes.push(ByteCode::GetGlobal(0, (constants.len() - 1) as u8));
                match lexer.next() {
                    Token::ParL => {
                        let code = match lexer.next() {
                            Token::Nil => ByteCode::LoadNil(1),
                            Token::True => ByteCode::LoadBool(1, true),
                            Token::False => ByteCode::LoadBool(1, false),
                            Token::Integer(i) => {
                                if let Ok(ii) = i16::try_from(i) {
                                    ByteCode::LoadInt(1, ii)
                                } else {
                                    load_const(&mut constants, 1, Value::Integer(i))
                                }
                            },
                            Token::Float(f) => load_const(&mut constants, 1, Value::Float(f)),
                            Token::String(s) => load_const(&mut constants, 1, Value::String(s)),
                            _ => {
                                panic!("invalid argument")
                            }
                        };
                        byte_codes.push(code);
                        match lexer.next() {
                            Token::ParR => {
                            }
                            _ => {
                                panic!("expected `)`");
                            }
                        }
                    },
                    Token::String(s) => {
                        constants.push(Value::String(s));
                        byte_codes.push(ByteCode::LoadGlobal(1, (constants.len() - 1) as u8));
                        byte_codes.push(ByteCode::Call(0, 1));
                    },
                    _ => panic!("expected string"),
                }
            }
            Token::EOS => {
                break;
            }
            _ => {
                panic!("unexpected token")
            }
        }
    }
    dbg!(&constants);
    dbg!(&byte_codes);
    ParseProto {
        constants,
        byte_codes,
    }
}

fn add_const(constants: &mut Vec<Value>, c: Value) -> usize {
    let constants = constants;
        constants.iter().position(|v| v == &c)
            .unwrap_or_else(|| {
                constants.push(c);
                constants.len() - 1
            })
}


fn load_const(constants: &mut Vec<Value>, dst: usize, c: Value) -> ByteCode {
    ByteCode::LoadConst(dst as u8, add_const(constants, c) as u8)
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::String(s) => write!(f, "{s}"),
            Value::Function(_) => write!(f, "function"),
            Value::Boolean(b) => write!(f, "{b}"),
            Value::Integer(i) => write!(f, "{i}"),
            Value::Float(n) => write!(f, "{n:?}"),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {

            match (self, other) {
                (Value::Nil, Value::Nil) => true,
                (Value::Boolean(b1), Value::Boolean(b2)) => *b1 == *b2,
                (Value::Integer(i1), Value::Integer(i2)) => *i1 == *i2,
                (Value::Float(f1), Value::Float(f2)) => *f1 == *f2,
                (Value::String(s1), Value::String(s2)) => *s1 == *s2,
                (Value::Function(f1), Value::Function(f2)) => std::ptr::eq(f1, f2),
                (_, _) => false,
            }
        
    }
}
