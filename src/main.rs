use std::io;
use std::io::prelude::*;
use std::fs::{self, File};
use std::path::Path;

static EXAMPLE_PATH: &'static str = "example";

#[derive(PartialEq, Debug, Clone)]
pub enum Symbol {
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Point,
    Semicolon,
    Colon,
    Minus,
    Lt,
    Bt,
    Equal,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Identifier(String),
    Number(f64),
    String(String),
    Symbol(Symbol),
    Arrow,
    Newline,
    Whitespace
}

#[derive(Clone)]
pub struct Scanner {
    filename: String,
    buf: Vec<char>,
    pos: usize,
    sym: Vec<String>,
}

impl Scanner {
    fn new(filename: String) -> Self {
        let buf = open(&filename).expect("error when reading the file");
        Self {
            filename: filename,
            buf: buf.chars().collect(),
            pos: 0,
            sym: Vec::new()
        }
    }
    fn lex_string(&mut self) -> Option<(Token, usize)> {
        let start = self.pos;
        let mut end = self.pos;
        while let Some(&c) = self.buf.get(end) {
            if c != '"' {
                break;
            } else {
                end += 1;
            }
        }
        if self.pos >= self.buf.len() {
            None
        } else {
            let range = start..end-1;
            Some((Token::String(self.buf[range].iter().collect()), end-1-start))
        }
    }
    fn lex_number(&mut self) -> Option<(Token, usize)> {
        let start = self.pos;
        let mut end = self.pos;
        let mut point_count = 0;
        while self.pos < self.buf.len() {
            match self.buf.get(end) {
                Some('.') => {
                    if point_count > 1 {
                        break;
                    } else {
                        point_count += 1;
                    }
                    end += 1;
                }
                Some(c) if c.is_numeric() => {
                    end += 1;
                }
                _ => {
                    break;
                }
            }
        }
        if self.pos >= self.buf.len() {
            None
        } else {
            let range = start..end;
            let tmp: String = self.buf[range].iter().collect();
            Some((Token::Number(tmp.parse::<f64>().unwrap()), end-start))
        }
    }
    fn lex_identifier(&mut self) -> Option<(Token, usize)> {
        let start = self.pos;
        let mut end = self.pos;
        while self.pos < self.buf.len() {
            match self.buf.get(end) {
                Some(c) if c.is_alphabetic() || c == &'_' => {
                    end += 1;
                }
                _ => {
                    break;
                }
            }
        }
        if self.pos >= self.buf.len() {
            None
        } else {
            let range = start..end;
            Some((Token::Identifier(self.buf[range].iter().collect()), end-start))
        }
    }
}

impl Iterator for Scanner {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if self.pos >= self.buf.len() {
            return None;
        }
        let c = self.buf.get(self.pos).unwrap().clone();
        match c {
            ' ' | '\t' => {
                self.pos += 1;
                while self.pos < self.buf.len() {
                    let d = self.buf.get(self.pos).unwrap();
                    if d == &' ' || d == &'\t' {
                        self.pos += 1;
                    } else {
                        break;
                    }
                }
                Some(Token::Whitespace)
            }
            '"' => {
                let (tok, len) = self.lex_string().expect("parsing error: string start");
                self.pos += len;
                Some(tok)
            }
            ':' => {
                self.pos += 1;
                Some(Token::Symbol(Symbol::Colon))
            }
            ';' => {
                while self.pos < self.buf.len() && self.buf.get(self.pos).unwrap() != &'\n' {
                    self.pos += 1;
                }
                Some(Token::Whitespace)
            }
            ',' => {
                self.pos += 1;
                Some(Token::Symbol(Symbol::Comma))
            }
            '.' => {
                self.pos += 1;
                Some(Token::Symbol(Symbol::Point))
            }
            '(' => {
                self.pos += 1;
                Some(Token::Symbol(Symbol::LeftParen))
            }
            ')' => {
                self.pos += 1;
                Some(Token::Symbol(Symbol::RightParen))
            }
            '[' => {
                self.pos += 1;
                Some(Token::Symbol(Symbol::LeftBracket))
            }
            ']' => {
                self.pos += 1;
                Some(Token::Symbol(Symbol::RightBracket))
            }
            '{' => {
                self.pos += 1;
                Some(Token::Symbol(Symbol::LeftBrace))
            }
            '}' => {
                self.pos += 1;
                Some(Token::Symbol(Symbol::RightBrace))
            }
            '-' => {
                self.pos += 1;
                if self.pos < self.buf.len() && self.buf[self.pos] == '>' {
                    self.pos += 1;
                    Some(Token::Arrow)
                } else {
                    Some(Token::Symbol(Symbol::Minus))
                }
            }
            '>' => {
                self.pos += 1;
                Some(Token::Symbol(Symbol::Lt))
            }
            '=' => {
                self.pos += 1;
                Some(Token::Symbol(Symbol::Equal))
            }
            '\n' => {
                self.pos += 1;
                Some(Token::Newline)
            }
            c if c.is_alphabetic() => {
                let (tok, len) = self.lex_identifier().expect("parsing error: identifier");
                self.pos += len;
                Some(tok)
            }
            c if c.is_numeric() => {
                let (tok, len) = self.lex_number().expect("parsing error: number");
                self.pos += len;
                Some(tok)
            }
            c => {
                println!("unmatch: {}", c);
                None
            }
        }
    }
}

fn open(path: &str) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents)
}

#[derive(Debug, Clone)]
pub enum AST {
    Block { name: String, content: Vec<AST> },
    Exist { name: String },
    Leaf { name: String },
    LeafDef { target: Box<AST>, stmt: Box<AST> },
    Edge { from: String, to: String },
    EdgeDef { target: Box<AST>, stmt: Box<AST> },
    Struct(Vec<AST>),
    Message { name: String, args: Vec<AST> },
    String(String),
    Number(f64)
}

pub struct Parser {
    cur: usize,
    toks: Vec<Token>
}

impl Parser {
    fn new(toks: Vec<Token>) -> Self {
        Self {
            cur: 0,
            toks: toks
        }
    }
    fn skip_blank(&mut self) {
      for i in self.cur..self.toks.len() {
        match self.toks[i] {
          Token::Newline => {},
          Token::Whitespace => {},
          _ => {
            self.cur = i;
            break;
          }
        }
      }
    }
    fn skip_whitespace(&mut self) {
        for i in self.cur..self.toks.len() {
            match self.toks[i] {
                Token::Whitespace => {
                },
                _ => {
                    self.cur = i;
                    break;
                }
            }
        }
    }
    fn parse_block<'a>(&mut self) -> Result<AST<'a>, String> {
        self.skip_blank();
        let first = self.toks[self.cur].clone();
        println!("{:?}", first);
        match first {
            Token::Symbol(Symbol::LeftBracket) => {
                self.cur += 1;
            },
            _ => {
                panic!("parsing error: expect [");
            }
        }
        let second = self.toks[self.cur].clone();
        self.cur += 1;
        let name = match second {
            Token::Identifier(name) => Some(name),
            _ => None
        };
        let name = name.expect("parsing error: expect identifier");
        let third = self.toks[self.cur].clone();
        self.cur += 1;
        match third {
            Token::Symbol(Symbol::RightBracket) => {
                let content = try!(self.parse_content());
                let block = AST::Block { name: name, content: content };
                Ok(block)
            },
            _ => {
                panic!("parsing error: expect ]");
            }
        }
    }
    fn parse_content<'a>(&mut self) -> Result<Vec<&'a AST<'a>>, String> {
        let target = try!(self.parse_target());
        println!("{:?}", target);
        unimplemented!();
    }
    fn parse_target<'a>(&mut self) -> Result<AST<'a>, String> {
        self.skip_blank(); 
        let first = self.toks[self.cur].clone();
        println!("{:?}", first);
        let mut leaf_name = String::new();
        self.cur += 1;
        match first {
            Token::Identifier(name) => { leaf_name = name },
            _ => panic!("parsing error: expect identifier")
        }
        let cur = self.cur;
        self.skip_blank();
        let second = self.toks[self.cur].clone();
        self.cur += 1;
        match second {
            Token::Arrow => {
                self.skip_blank();
                let third = self.toks[self.cur].clone();
                self.cur += 1;
                match third {
                    Token::Identifier(name) => {
                        Ok(AST::Edge { from: leaf_name, to: name })
                    }
                    _ => panic!("parsing error: expect identifier")
                }
            },
            _ => {
                Ok(AST::Leaf { name: leaf_name })
            }
        }
    }

    fn parse<'a>(&mut self) -> Result<Vec<&'a AST<'a>>, String> {
        unimplemented!()
    }
}

fn test_by_examples() -> io::Result<()> { // = Result<(), io::Error>
    let example_path = Path::new(EXAMPLE_PATH);
    if example_path.is_dir() {
        for entry in fs::read_dir(&example_path)? {
            let entry = entry?;
            let path = entry.path();
            path.to_str().map_or_else(|| {
                println!("error when getting the file name");
            },
            |path_str| {
                //let content = open(path_str).expect("error when reading the file");
                //println!("{}", content);
                let scanner = Scanner::new(path_str.to_string());
                let sym: Vec<Token> = scanner.into_iter().collect();
                println!("{:?}", sym);
                let mut parser = Parser::new(sym);
                parser.parse_block();
                /*
                for tok in scanner {
                    println!("{:?}", tok);
                }
                */
            });
        }
    }
    Ok(())
}

fn main() {
    test_by_examples().unwrap();
}
