use gmml::parse;
use std::env;
use std::fs;

fn main() {
    let mut iter = env::args();
    iter.next();
    for argument in iter {
        println!("{}", argument);
        let scanner = parse::Scanner::new(argument.to_string());
        let sym: Vec<parse::Token> = scanner.into_iter().collect();
        let mut parser = parse::Parser::new(sym);
        let result = parser.parse().expect("failed to parse");
        println!("{:#?}\n", result);
    }
}
