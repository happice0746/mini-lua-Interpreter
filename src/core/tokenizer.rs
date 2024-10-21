use std::fs::File;

#[derive(Debug)]
pub enum Token {
  Name(String),
  String(String),
  EOS,
}

#[derive(Debug)]
pub struct Tokenizer {
  input: File,
  tokens: Vec<Token>
}

impl Tokenizer {
  pub fn new(input: File)-> Self {
    let tokens: Vec<Token> = vec![];
    // ....
    Tokenizer {
      input,
      tokens
    }
  }
  pub fn next(self)-> Token {
    self.tokens.into_iter().next().unwrap()
  }
}
