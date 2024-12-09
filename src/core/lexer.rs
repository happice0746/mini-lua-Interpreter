use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::mem;

#[derive(Debug, PartialEq)]
pub enum Token {
    // keywords
    And,
    Do,
    Else,
    Elseif,
    End,
    False,
    True,
    For,
    Function,
    Goto,
    If,
    In,
    Local,
    Nil,
    Not,
    Or,
    Repeat,
    Return,
    Then,
    Until,
    While,

    // operator
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Len,
    BitAnd,
    BitXor,
    BitOr,
    ShiftL,
    ShiftR,
    Idiv,
    NotEq,
    Equal,
    LesEq,
    GreEq,
    Less,
    Greater,
    Assign,
    ParL,
    ParR,
    CurlyL,
    CurlyR,
    SqurL,
    SqurR,
    DoubColon,
    SemiColon,
    Colon,
    Comma,
    Dot,
    Concat,
    Dots,
    Name(String),
    Integer(i64),
    Float(f64),
    String(String),
    EOS,
}

#[derive(Debug)]
pub struct Lexer {
    input: File,
    ahead: Token
}

impl Lexer {
    pub fn new(input: File) -> Self {
        Lexer { 
            input,
            ahead: Token::EOS
        }
    }
    pub fn next(&mut self) -> Token {
        if self.ahead == Token::EOS {
            self.do_next()
        } else {
            mem::replace(&mut self.ahead, Token::EOS)
        }
    }

    pub fn peek(&mut self) -> &Token {
        if self.ahead == Token::EOS {
            self.ahead = self.do_next();
        }
        &self.ahead
    }

    pub fn do_next(&mut self) -> Token {
        let ch = self.read_char();
        match ch {
            '+' => Token::Add,
            '*' => Token::Mul,
            '%' => Token::Mod,
            '^' => Token::Pow,
            '#' => Token::Len,
            '&' => Token::BitAnd,
            '|' => Token::BitOr,
            '(' => Token::ParL,
            ')' => Token::ParR,
            '[' => Token::CurlyL,
            ']' => Token::CurlyR,
            '{' => Token::SqurL,
            '}' => Token::SqurR,
            ';' => Token::SemiColon,
            ',' => Token::Comma,
            '\0' => Token::EOS,
            '\'' | '"' => self.read_string(ch),
            '-' => {
                if self.read_char() == '-' {
                    self.read_comment();
                    self.next()
                } else {
                    self.back_char();
                    Token::Sub
                }
            },
            ':' => self.check_ahead_condition(':', Token::DoubColon, Token::Colon),
            '~' => self.check_ahead_condition('=', Token::NotEq, Token::BitXor),
            '=' => self.check_ahead_condition('=', Token::Equal, Token::Assign),
            '/' => self.check_ahead_condition('/', Token::Idiv, Token::Div),
            '>' => self.check_ahead_conditions('>', Token::ShiftR, '=', Token::GreEq, Token::Greater),
            '<' => self.check_ahead_conditions('<', Token::ShiftL, '=', Token::LesEq, Token::Less),
            ' ' | '\r' | '\n' | '\t' => self.next(),
            'a'..='z' | 'A'..='Z' | '_' => self.read_name(ch),
            '0'..='9' => self.read_number(ch),
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

    fn back_char(&mut self) -> () {
        self.input.seek(SeekFrom::Current(-1)).unwrap();
    }
    
    fn check_ahead_condition(&mut self, ahead: char, long: Token, short: Token) -> Token {
        if self.read_char() == ahead {
            long
        } else {
            self.back_char();
            short
        }
    }

    fn check_ahead_conditions(&mut self, ahead1: char, long1: Token, ahead2: char, long2: Token, short: Token) -> Token {
        let ch = self.read_char();
        if ch == ahead1 {
            long1
        } else if ch == ahead2 {
            long2
        } else {
            self.back_char();
            short
        }
    }

    fn read_string (&mut self, quote: char) -> Token {
        let mut s = String::new();
        loop {
            let ch = self.read_char();
            match ch {
                '\0' => panic!("unfinished literal string"),
                ch if quote == ch => break,
                ch => s.push(ch),
            }
        }
        Token::String(s)
    }

    fn read_comment(&mut self) -> Token {
        todo!()
    }

    fn read_number(&mut self, ch: char) -> Token {
        if ch == '0' {
            let next_ch = self.read_char();
           if next_ch == 'x' || next_ch == 'X' {
               self.handle_hex();
            } 
            self.back_char();
        }
        let mut sum = char::to_digit(ch, 10).unwrap();
        loop {
            let next_ch =  self.read_char();
            if let Some(n) = next_ch.to_digit(10) {
                sum = sum * 10 + n;
            } else if next_ch == '.' {
                return self.handle_fraction(sum as i64);
            } else if next_ch == 'e' || next_ch == 'E' {
                return self.handle_exp();
            } else {
                self.back_char();
                break;
            }
        }
        Token::Integer(sum as i64)
    }

    fn handle_hex(&mut self) {
        let n = char::to_digit(self.read_char(), 16).unwrap();
    }

    fn handle_fraction(&mut self, int_part: i64) -> Token {
        Token::Integer(int_part)
    }

    fn handle_exp(&mut self) -> Token {
       todo!()
    }

    fn read_name(&mut self, ch: char) -> Token {
        let mut name = String::new();
        name.push(ch);
        loop {
            match self.read_char() {
                '\0' => break,
                '_' => name.push('_'),
                ch if ch.is_alphanumeric() => name.push(ch),
                _ => {
                    self.back_char();
                    break;
                }
            }
        }
        match &name as &str {
            "and" => Token::And,
            "or" => Token::Or,
            "if" => Token::If,
            "else" => Token::Else,
            "elseif" => Token::Elseif,
            "end" => Token::End,
            "for" => Token::For,
            "function" => Token::Function,
            "goto" => Token::Goto,
            "in" => Token::In,
            "local" => Token::Local,
            "not" => Token::Not,
            "repeat" => Token::Repeat,
            "return" => Token::Return,
            "then" => Token::Then,
            "until" => Token::Until,
            "while" => Token::While,
            "do" => Token::Do,
            "true" => Token::True,
            "false" => Token::False,
            "Nil" => Token::Nil,
            _ => Token::Name(name)
        }
        
    }
}
