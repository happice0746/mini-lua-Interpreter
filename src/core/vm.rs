use std::collections::HashMap;

use super::parser::Value;

pub struct ExeState {
  globals: HashMap<String, Value>,
  stack: Vec::<Value>,
}

impl ExeState {
  fn new(self) -> Self {
    let mut globals: HashMap<String, Value> = HashMap::new();
    globals.insert(String::from("print"), Value::Function(lib_print));
    return ExeState {
      globals,
      stack: vec![]
    }
  }
}

fn lib_print(state: &mut ExeState) -> i32 {
  print!("{:?}", state.stack[0]);
  0
}