use std::io;
use std::io::prelude::*;
use std::fs::{self, File};
use std::path::Path;
use std::collections::HashMap;
use std::convert::From;

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
        let start = self.pos+1;
        let mut end = self.pos+1;
        while let Some(&c) = self.buf.get(end) {
            if c == '"' {
                break;
            } else {
                end += 1;
            }
        }
        if end > self.buf.len() {
            None
        } else {
            let range = start..end;
            Some((Token::String(self.buf[range].iter().collect()), end-start+2))
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
        if end >= self.buf.len() {
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
        if end >= self.buf.len() {
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
    LeafDef { target: Box<AST>, stmt: Box<AST> },
    Edge { from: Box<AST>, to: Box<AST> },
    EdgeDef { target: Box<AST>, stmt: Box<AST> },
    Struct(Vec<AST>),
    Message { name: String, args: Vec<AST> },
    String(String),
    Number(f64),
    Symbol(String),
    List(Vec<AST>)
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
    fn parse_block(&mut self) -> Result<AST, String> {
        self.skip_blank();
        let first = self.toks[self.cur].clone();
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
                self.skip_whitespace();
                let fourth = self.toks[self.cur].clone();
                self.cur += 1;
                match fourth {
                  Token::Newline => {
                    let content = try!(self.parse_content());
                    let block = AST::Block { name: name, content: content };
                    Ok(block)
                  },
                  _ => panic!("parsing error: expect newline")
                }
            },
            _ => {
                panic!("parsing error: expect ]");
            }
        }
    }
    fn parse_content(&mut self) -> Result<Vec<AST>, String> {
        let mut content: Vec<AST> = Vec::new();
        while self.cur < self.toks.len() {
          let head = self.toks[self.cur].clone();
          match head {
            Token::Symbol(Symbol::LeftBracket) => {
              break;
            },
            Token::Newline => {
              self.cur += 1;
              break;
            }
            Token::Identifier(_) | Token::Number(_) | Token::String(_) => {
              let first = try!(self.parse_target());
              self.skip_whitespace();
              let second = self.toks[self.cur].clone();
              self.cur += 1;
              match second {
                Token::Newline => {
                  content.push(first);
                },
                Token::Symbol(Symbol::Colon) => {
                  match &first {
                    AST::Edge{from:_,to:_} => {
                      self.skip_blank();
                      let third = try!(self.parse_value());
                      self.skip_whitespace();
                      let fourth = self.toks[self.cur].clone();
                      self.cur += 1;
                      match fourth {
                        Token::Newline => {
                          let target = first.clone();
                          let stmt = third.clone();
                          content.push(AST::EdgeDef {target: Box::new(target), stmt: Box::new(stmt)});
                        },
                        _ => panic!("parsing error: expect newline")
                      }
                    },
                    _ => panic!("parsing error: edge : stmt ?")
                  }
                },
                Token::Symbol(Symbol::Equal) => {
                  match &first {
                    AST::Symbol(_) => {
                      self.skip_blank();
                      let third = try!(self.parse_value());
                      self.skip_whitespace();
                      let fourth = self.toks[self.cur].clone();
                      self.cur += 1;
                      match fourth {
                        Token::Newline => {
                          let target = first.clone();
                          let stmt = third.clone();
                          content.push(AST::LeafDef {target: Box::new(target), stmt: Box::new(stmt)});
                        },
                        _ => panic!("parsing error: expect newline")
                      }
                    },
                    _ => panic!("parsing error: leaf = stmt ?")
                  }
                },
                _ => panic!("parsing error: expect : or newline")
            }
          }
          _ => panic!("parsing error: expect newline or identifier")
        }
      }
       Ok(content)
    }

    fn parse_key(&mut self) -> Result<AST, String> {
        let first = self.toks[self.cur].clone();
        self.cur += 1;
        match first {
            Token::Identifier(name) => Ok(AST::Symbol(name.to_string())),
            Token::String(string) => Ok(AST::String(string.to_string())),
            Token::Number(number) => Ok(AST::Number(number)),
            _ => panic!("parsing error: expect identifier or string or number")
        }
    }

    fn parse_target(&mut self) -> Result<AST, String> {
        let left = try!(self.parse_key());
        let cur = self.cur;
        self.skip_blank();
        let second = self.toks[self.cur].clone();
        self.cur += 1;
        match second {
            Token::Arrow => {
                self.skip_blank();
                let right = try!(self.parse_key());
                Ok(AST::Edge { from: Box::new(left), to: Box::new(right) })
            },
            _ => {
                self.cur = cur;
                Ok(left)
            }
        }
    }

    fn parse_value(&mut self) -> Result<AST, String> {
      let first = self.toks[self.cur].clone();
      match first {
        Token::Symbol(Symbol::LeftBrace) => self.parse_struct(),
        Token::String(string) => {
          self.cur += 1;
          Ok(AST::String(string))
        },
        Token::Number(number) => {
          self.cur += 1;
          Ok(AST::Number(number))
        },
        Token::Identifier(_) => self.parse_message(),
        _ => panic!("parsing error: expect {, string, number, or identifier")
      }
    }

    fn parse_pair(&mut self) -> Result<AST, String> {
      let pair_left = try!(self.parse_key());
      self.skip_blank();
      let second = self.toks[self.cur].clone();
      self.cur += 1;
      match second {
        Token::Symbol(Symbol::Colon) => {},
        _ => panic!("parsing error: expect :")
      }
      self.skip_blank();
      Ok(AST::LeafDef { target: Box::new(pair_left), stmt: Box::new(try!(self.parse_value()))})
    }

    fn parse_struct(&mut self) -> Result<AST, String> {
      let first = self.toks[self.cur].clone();
      self.cur += 1;
      match first {
        Token::Symbol(Symbol::LeftBrace) => {},
        _ => panic!("parsing error: expect {")
      }
      let mut content: Vec<AST> = Vec::new();
      self.skip_blank();
      while self.cur < self.toks.len() {
        let second = self.toks[self.cur].clone();
        match second {
          Token::Symbol(Symbol::RightBrace) => {
            break;
          },
          _ => {}
        }
        let pair = try!(self.parse_pair());
        content.push(pair);
        self.skip_blank();
        let comma = self.toks[self.cur].clone();
        match comma {
          Token::Symbol(Symbol::Comma) => {
            self.cur += 1;
            self.skip_blank();
          },
          _ => {
            break;
          }
        }
      }
      let third = self.toks[self.cur].clone();
      self.cur += 1;
      match third {
        Token::Symbol(Symbol::RightBrace) => {
          Ok(AST::Struct(content) )
        },
        _ => panic!("parsing error: expect }")
      }
    }

    fn parse_message(&mut self) -> Result<AST, String> {
      let first = self.toks[self.cur].clone();
      self.cur += 1;
      let message_name: String;
      match first {
        Token::Identifier(name) => {
          message_name = name;
        },
        _ => panic!("parsing error: expect identifier")
      }
      let second = self.toks[self.cur].clone();
      match second {
        Token::Symbol(Symbol::LeftParen) => Ok(AST::Message {name: message_name, args: try!(self.parse_args())}),
        _ => {
          Ok(AST::Symbol(message_name))
        }
      }
    }

    fn parse_args(&mut self) -> Result<Vec<AST>, String> {
      let first = self.toks[self.cur].clone();
      self.cur += 1;
      match first {
        Token::Symbol(Symbol::LeftParen) => {},
        _ => panic!("parsing error: expect (")
      }
      let mut content: Vec<AST> = Vec::new();
      self.skip_blank();
      while self.cur < self.toks.len() {
        let second = self.toks[self.cur].clone();
        match second {
          Token::Symbol(Symbol::RightParen) => {
            break;
          },
          _ => {}
        }
        let value = try!(self.parse_value());
        content.push(value);
        self.skip_blank();
        let comma = self.toks[self.cur].clone();
        match comma {
          Token::Symbol(Symbol::Comma) => {
            self.cur += 1;
            self.skip_blank();
          },
          _ => {
            break;
          }
        }
      }
      let third = self.toks[self.cur].clone();
      self.cur += 1;
      match third {
        Token::Symbol(Symbol::RightParen) => {
          Ok(content)
        },
        _ => panic!("parsing error: expect )")
      }
    }

    fn get_ast(&mut self) -> Result<Vec<AST>, String> {
      let mut blocks: Vec<AST> = Vec::new();
      while self.cur < self.toks.len() {
        let block = try!(self.parse_block());
        blocks.push(block);
      }
      Ok(blocks)
    }

    fn parse(&mut self) -> Result<HashMap<String, GValue>, String> {
        let root_ast = try!(self.get_ast());
        let mut root: HashMap<String, GValue> = HashMap::new();
        let blocks: Vec<GValue> = root_ast.into_iter().map(GValue::from).collect();
        for block in blocks {
            match block {
                GValue::Pair(key, value) => {
                    match *key {
                        GValue::String(key_string) => {
                            root.entry(key_string).or_insert(*value);
                        },
                        _ => panic!("convert error: key should be String")
                    }
                },
                _ => panic!("convert error: invalid block syntax")
            }
        }
        Ok(root)
    }
}

#[derive(Debug)]
pub enum GValue {
    String(String),
    Number(f64),
    Symbol(String),
    Message(String, Vec<GValue>),
    Edge(Box<GValue>, Box<GValue>),
    Map(HashMap<String, GValue>),
    Vec(Vec<GValue>),
    Pair(Box<GValue>, Box<GValue>)
}

impl From<AST> for GValue {
    fn from(ast: AST) -> Self {
        match ast {
            AST::String(string) => GValue::String(string.to_string()),
            AST::Number(number) => GValue::Number(number),
            AST::Symbol(name) => GValue::Symbol(name.to_string()),
            AST::LeafDef{target,stmt} => GValue::Pair(Box::new(GValue::from(*target)), Box::new(GValue::from(*stmt))),
            AST::Edge{from,to} => GValue::Edge(Box::new(GValue::from(*from)), Box::new(GValue::from(*to))),
            AST::EdgeDef{target,stmt} => GValue::Pair(Box::new(GValue::from(*target)), Box::new(GValue::from(*stmt))),
            AST::Message{name,args} => GValue::Message(name, args.into_iter().map(GValue::from).collect()),
            AST::Struct(content) => GValue::Vec(content.into_iter().map(GValue::from).collect()),
            AST::Block{name, content} =>
                GValue::Pair(Box::new(GValue::String(name.to_string())),
                Box::new(GValue::Vec(content.into_iter().map(GValue::from).collect())))
        }
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
                let mut parser = Parser::new(sym);
                let result = parser.parse().expect("failed to parse");
                println!("{:?}", result);
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
