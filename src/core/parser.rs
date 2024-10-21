use core::panic;
use std::{fmt, fs::File};

use super::{tokenizer::{Token, Tokenizer}, vm::ExeState};

pub enum Value {
  Nil,
  String(String),
  Function(fn (&mut ExeState) -> i32)
}
#[derive(Debug)]
pub enum ByteCode {
  GetGlobal(u8, u8),
  LoadGlobal(u8, u8),
  Call(u8, u8)
}

#[derive(Debug)]
pub struct ParseProto {
  pub constants: Vec::<Value>,
  pub byte_codes: Vec::<ByteCode>,
}

pub fn load(input: File) -> ParseProto {
  let mut constants: Vec<Value> = vec![];
  let mut byte_codes: Vec<ByteCode> = vec![];
  let mut tokenizer = Tokenizer::new(input);
  loop {
    match tokenizer.next() {
        Token::Name(name) => {
          constants.push(Value::String(name));
          byte_codes.push(ByteCode::GetGlobal(0, (constants.len()-1) as u8));
          if let Token::String(s) = tokenizer.next() {
            constants.push(Value::String(s));
            byte_codes.push(ByteCode::LoadGlobal(1, (constants.len()-1) as u8));
            byte_codes.push(ByteCode::Call(0, 1));
          } else {
            panic!("expected string")
          }
        },
        Token::EOS => {
          break;
        },
        _ => {
          panic!("unexpected token")
        }
    }
  }
  dbg!(&constants);
  dbg!(&byte_codes);
  ParseProto { constants, byte_codes }
}

impl fmt::Debug for Value {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Value::Nil => write!(f, "nil"),
      Value::String(s) => write!(f, "{s}"),
      Value::Function(_) => write!(f, "function"),
    }
  }
}