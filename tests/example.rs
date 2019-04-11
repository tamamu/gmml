#[cfg(test)]
mod tests {
    use gmml::parse;
    use std::fs;
    use std::path::PathBuf;
    #[test]
    fn parse_example() {
        let mut example_path = PathBuf::new();
        example_path.push(env!("CARGO_MANIFEST_DIR"));
        example_path.push("example");
        if example_path.is_dir() {
            for entry in fs::read_dir(&example_path).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                path.to_str().map_or_else(
                    || {
                        println!("error when getting the file name");
                    },
                    |path_str| {
                        println!("{}:", path_str);
                        let scanner = parse::Scanner::new(path_str.to_string());
                        let sym: Vec<parse::Token> = scanner.into_iter().collect();
                        let mut parser = parse::Parser::new(sym);
                        let result = parser.parse().expect("failed to parse");
                        println!("{:#?}\n", result);
                    },
                );
            }
        }
    }
}
