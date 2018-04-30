use std::io;
use std::io::prelude::*;
use std::fs::{self, File};
use std::path::Path;

static EXAMPLE_PATH: &'static str = "example";

fn open(path: &str) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents)
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
                let content = open(path_str).expect("error when reading the file");
                println!("{}", content);
            });
        }
    }
    Ok(())
}

fn main() {
    test_by_examples();
}
