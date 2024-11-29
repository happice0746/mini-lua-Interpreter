use core::panic;
use std::{cmp::Ordering, collections::HashMap};

use super::parser::{ByteCode, ParseProto, Value};

pub struct ExeState {
    globals: HashMap<String, Value>,
    stack: Vec<Value>,
    func_index: usize,
}

impl ExeState {
    pub fn new() -> Self {
        let mut globals: HashMap<String, Value> = HashMap::new();
        globals.insert(String::from("print"), Value::Function(lib_print));
        return ExeState {
            globals,
            stack: vec![],
            func_index: 0,
        };
    }

    fn set_stack(&mut self, dst: u8, v: Value) {
        let dst = dst as usize;
        match dst.cmp(&self.stack.len()) {
            Ordering::Equal => self.stack.push(v),
            Ordering::Less => self.stack[dst] = v,
            Ordering::Greater => panic!("fail in set_stack"),
        }
    }
    pub fn execute(&mut self, proto: &ParseProto) -> () {
        for bytecode in proto.byte_codes.iter() {
            match *bytecode {
                ByteCode::Call(func, _) => {
                    let func = &self.stack[func as usize];
                    if let Value::Function(f) = func {
                        f(self);
                    } else {
                        panic!("invalid function: {func:?}");
                    }
                }
                ByteCode::GetGlobal(dst, name) => {
                    let name = &proto.constants[name as usize];
                    if let Value::String(key) = name {
                        let v = self.globals.get(key).unwrap_or(&Value::Nil).clone();
                        self.set_stack(dst, v);
                    } else {
                        panic!("invalid global key: {name:?}");
                    }
                }
                ByteCode::LoadConst(dst, c) => {
                    let v = proto.constants[c as usize].clone();
                    self.set_stack(dst, v);
                },
                ByteCode::LoadNil(dst) => {
                    self.set_stack(dst, Value::Nil);
                }
                ByteCode::LoadBool(dst, b) => {
                    self.set_stack(dst, Value::Boolean(b));
                }
                ByteCode::LoadInt(dst, i) => {
                    self.set_stack(dst, Value::Integer(i as i64));
                }
                ByteCode::Move(dst, i ) => {
                    let v = proto.constants[i as usize].clone();
                    self.set_stack(dst, v);
                }
                ByteCode::SetGlobal(dst, i) => {
                    let name = proto.constants[dst as usize].clone();
                    if let Value::String(key) = name {
                        let value = self.stack[i as usize].clone();
                        self.globals.insert(key, value);
                    } else {
                        panic!("invalid global key:{name:?}");
                    }
                }
                ByteCode::SetGlobalConst(dst, i) => {
                    let name = proto.constants[dst as usize].clone();
                    if let Value::String(key) = name {
                        let value = proto.constants[i as usize].clone();
                        self.globals.insert(key, value);
                    } else {
                        panic!("invalid global key:{name:?}");
                    }
                }
                ByteCode::SetGlobalGlobal(dst, i) => {
                    let name = proto.constants[dst as usize].clone();
                    if let Value::String(key) = name {
                        let i = proto.constants[i as usize].clone();
                        if let Value::String(is) = i {
                            let value = self.globals.get(&is).unwrap_or(&Value::Nil).clone();
                            self.globals.insert(key, value);
                        }
                    } else {
                        panic!("invalid global key:{name:?}");
                    }
                }
                _ => {
                    panic!("invalid bytecodes")
                }
            }
        }
    }
}

fn lib_print(state: &mut ExeState) -> i32 {
    print!("{:?}\n", state.stack[state.func_index+1]);
    0
}
