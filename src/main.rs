use pest::Parser;

use std::fs::{read_to_string, read_dir};

#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "apex-grammar.pest"] // relative to src
struct ApexParser;

fn main() {
    for entry in read_dir("examples").unwrap() {
        let entry = entry.unwrap();
        let path = &entry.path();
        if path.is_file() {
            let contents = read_to_string(&path).unwrap();
            println!("Parsing file: {:?}", &path);
            display_parse(&contents);
        }
    }
}

fn display_parse(input: &str) {
    let pairs = match ApexParser::parse(Rule::class, input) {
        Ok(r) => r,
        Err(err) => {
            println!("\t{}", err);
            return;
        }
    };
    for pair in pairs {
        println!("\tPair: {:?}", pair);
    }
}

#[cfg(test)]
mod test {

    use super::*;

    // TODO: Upgrade these to use the new "matches!" macro

    #[test]
    fn parses_integers() {
        ApexParser::parse(Rule::num, "124").unwrap();
    }
}