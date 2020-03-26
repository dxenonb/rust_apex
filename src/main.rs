use pest::Parser;

use std::fs::{read_to_string, read_dir};

#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "apex-grammar.pest"] // relative to src
struct ApexParser;

fn main() {

    let mut errors = Vec::new();
    let mut success = 0;

    for entry in read_dir("examples").unwrap() {
        let entry = entry.unwrap();
        let path = &entry.path();
        if path.is_file() {
            let contents = read_to_string(&path).unwrap();
            println!("Parsing file: {:?}", &path);
            if let Err(err) = display_parse(&contents) {
                errors.push(err);
            } else {
                success += 1;
            }
        }
    }

    // TODO: oops forgot to track file name
    for error in &errors {
        println!("Error: {}", error);
    }

    let total = success + errors.len();

    println!("Parsed {} files.", total);
    println!("\tparsing: {}", success);
    println!("\tfailing: {}", errors.len());

    if !errors.is_empty() {
        println!("\nSee above for errors.");
    }
}

fn display_parse(input: &str) -> Result<(), String> {
    let _pairs = match ApexParser::parse(Rule::class, input) {
        Ok(r) => r,
        Err(err) => {
            return Err(format!("{}", err));
        }
    };
    // TODO: we probably don't actually want to print all this... or do we?
    // for pair in pairs {
    //     println!("\tPair: {:?}", pair);
    // }
    Ok(())
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