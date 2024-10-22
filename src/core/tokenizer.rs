use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug)]
pub enum Token {
    Name(String),
    String(String),
    EOS,
}

#[derive(Debug)]
pub struct Tokenizer {
    input: File,
}

impl Tokenizer {
    pub fn new(input: File) -> Self {
        Tokenizer { input }
    }
    pub fn next(&mut self) -> Token {
        let ch = self.read_char();
        match ch {
            '"' => {
                let mut s = String::new();
                loop {
                    match self.read_char() {
                        '\0' => panic!("unfinished literal string"),
                        '"' => break,
                        ch => s.push(ch),
                    }
                }
                Token::String(s)
            }
            ' ' | '\r' | '\n' | '\t' => self.next(),
            '\0' => Token::EOS,
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                let mut name = String::new();
                name.push(ch);
                loop {
                    match self.read_char() {
                        '\0' => break,
                        '_' => name.push('_'),
                        ch if ch.is_alphanumeric() => name.push(ch),
                        _ => {
                            self.input.seek(SeekFrom::Current(-1)).unwrap();
                            break;
                        }
                    }
                }
                Token::Name(name)
            }
            _ => {
                panic!("expected char")
            }
        }
    }
    pub fn read_char(&mut self) -> char {
        let mut buf: [u8; 1] = [0];
        if self.input.read(&mut buf).unwrap() == 1 {
            buf[0] as char
        } else {
            '\0'
        }
    }
}
