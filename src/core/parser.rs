use core::panic;
use std::{fs::File, vec};

use super::{
    lexer::{Lexer, Token},
    vm::ExeState,
};

#[derive(Clone, Debug, PartialEq)]
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
    SetGlobalConst(u8, u8),
    SetGlobalGlobal(u8, u8),
    SetGlobal(u8, u8),
    GetGlobal(u8, u8),
    LoadInt(u8, i16),
    LoadBool(u8, bool),
    LoadConst(u8, u8),
    LoadNil(u8),
    Call(u8, u8),
    Move(u8, u8),
}

#[derive(Debug)]
pub struct ParseProto {
    pub constants: Vec<Value>,
    pub byte_codes: Vec<ByteCode>,
    pub locals: Vec<String>,
    pub lexer: Lexer,
}

impl ParseProto{
    pub fn load(input: File) -> ParseProto {
        let mut proto = ParseProto {
            constants: vec![],
            byte_codes: vec![],
            locals: vec![],
            lexer: Lexer::new(input),
        };
        
        proto.chunk();
        dbg!(&proto.constants);
        dbg!(&proto.locals);
        dbg!(&proto.byte_codes);
        proto
    }

    fn chunk(&mut self) {
        loop {
            match self.lexer.next() {
                Token::Name(name) => {
                    match self.lexer.peek() {
                        &Token::Assign => {
                            self.load_assign(name);
                        }
                        _ => {
                            self.function_call(name);
                        }
                    }
                }
                Token::Local => {
                    self.load_local();
                }
                Token::EOS => break,
                _ => {

                }
            }
        }

    }
    
    fn add_const(&mut self, c: Value) -> usize {
        let constants = &mut self.constants;
        constants.iter().position(|v| v == &c)
            .unwrap_or_else(|| {
                constants.push(c);
                constants.len() - 1
            })
    }
    
    fn load_const(&mut self, dst: usize, c: Value) -> ByteCode {
        ByteCode::LoadConst(dst as u8, self.add_const(c) as u8)
    }

    fn load_exp(&mut self, dst: usize) {
        let code = match self.lexer.next() {
            Token::Float(f) => self.load_const(dst, Value::Float(f)),
            Token::Integer(i) => {
                if let Ok(ii) = i16::try_from(i) {
                    ByteCode::LoadInt(dst as u8, ii)
                } else {
                    self.load_const(dst, Value::Integer(i))
                }
            },
            Token::True => ByteCode::LoadBool(dst as u8, true),
            Token::False => ByteCode::LoadBool(dst as u8, false),
            Token::Nil => ByteCode::LoadNil(dst as u8),
            Token::String(s) => self.load_const(dst, Value::String(s)),
            Token::Name(var) => self.load_var(dst, var),
            _ => {
                panic!("expected expirement")
            }
        };
        self.byte_codes.push(code);
    }

    fn load_assign(&mut self, var: String) {
        self.lexer.next();
        if let Some(i) = self.get_local(&var) {
            self.load_exp(i)
        } else {
            let dst = self.add_const(Value::String(var)) as u8;
            let code = match self.lexer.next() {
                Token::Nil => ByteCode::SetGlobalConst(dst, self.add_const(Value::Nil) as u8),
                Token::True => ByteCode::SetGlobalConst(dst, self.add_const(Value::Boolean(true)) as u8),
                Token::False => ByteCode::SetGlobalConst(dst, self.add_const(Value::Boolean(false)) as u8),
                Token::Integer(i) => ByteCode::SetGlobalConst(dst, self.add_const(Value::Integer(i)) as u8),
                Token::Float(f) => ByteCode::SetGlobalConst(dst, self.add_const(Value::Float(f)) as u8),
                Token::String(s) => ByteCode::SetGlobalConst(dst, self.add_const(Value::String(s)) as u8),
                Token::Name(var) => {
                    if let Some(i) = self.get_local(&var) {
                        ByteCode::Move(dst, i as u8)
                    } else {
                        ByteCode::SetGlobalGlobal(dst, (self.add_const(Value::String(var))) as u8)
                    }
                }
                _ => {
                    panic!("invalid argument")
                }
            };
            self.byte_codes.push(code);
        }
    }

    fn load_local(&mut self) {
        let var = if let Token::Name(v) = self.lexer.next() {
            v
        } else {
            panic!("expcetd variable");
        };
        if self.lexer.next() != Token::Assign {
            panic!("expected assignment")
        }

        self.add_const(c)
    }

    fn get_local(&self, name: &str) -> Option<usize> {
        self.locals.iter().rposition(|v| v == name)
    }

    fn load_var(&mut self, dst: usize, name: String) -> ByteCode {
        if let Some(i) = self.get_local(&name) {
            ByteCode::Move(dst as u8, i as u8)
        } else {
            let ic = self.add_const(Value::String(name));
            ByteCode::GetGlobal(dst as u8, ic as u8)
        }   
    }

    fn function_call(&mut self, name: String) {
        let ifunc = self.locals.len();
        let iarg = ifunc + 1;
        let code = self.load_var(ifunc, name);
        self.byte_codes.push(code);

        match self.lexer.next() {
            Token::ParL => {
                self.load_exp(iarg);
                if self.lexer.next() == Token::ParR {
                    self.byte_codes.push(ByteCode::Call(ifunc as u8, iarg as u8));
                } else {
                    panic!("expected right parantheses")
                }
            }
            _ => {
                panic!("expected left parantheses")
            }
        }
    }


}
