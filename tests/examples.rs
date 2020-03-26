use apex::parse_class;

use std::fs::{read_to_string, read_dir};

#[test]
fn main() -> Result<(), ()> {
    let mut errors = Vec::new();
    let mut success = 0;
    let mut expected_failures = 0;

    for entry in read_dir("tests/examples").unwrap() {
        let entry = entry.unwrap();
        let path = &entry.path();
        if path.is_file() {
            let valid = path.to_str()
                .map(|s| s.contains(".valid"))
                .unwrap_or(false);
            let contents = read_to_string(&path).unwrap();

            let result = display_parse(&contents);

            if let Err(err) = result {
                if valid {
                    errors.push((path.clone(), err));
                } else {
                    expected_failures += 1;
                }
            } else {
                if valid {
                    success += 1;
                } else {
                    errors.push((path.clone(), "expected parse to fail".to_string()))
                }
            }
        }
    }

    for (file, error) in &errors {
        println!("Error in file: {:?}", file);
        println!("{}", error);
    }

    println!();

    let total = success + expected_failures + errors.len();

    println!("Parsed {} files.", total);
    println!("\t            valid: {}", success);
    println!("\t          invalid: {}", expected_failures);
    println!("\tunexpected errors: {}", errors.len());

    if !errors.is_empty() {
        println!("\nSee above for errors.");
        return Err(());
    }

    Ok(())
}

fn display_parse(input: &str) -> Result<(), String> {
    let _pairs = match parse_class(input) {
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
