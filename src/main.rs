mod parse;

use std::io;
use std::fs;
use std::path::Path;
use parse::{Parser, Scanner, Token};

static EXAMPLE_PATH: &'static str = "example";

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
                println!("{}:", path_str);
                let scanner = Scanner::new(path_str.to_string());
                let sym: Vec<Token> = scanner.into_iter().collect();
                let mut parser = Parser::new(sym);
                let result = parser.parse().expect("failed to parse");
                println!("{:?}\n", result);
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
