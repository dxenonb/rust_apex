use apex::parse_class;

use std::env;

use std::fs::{read_dir, read_to_string};
use std::path::{PathBuf, Path};

struct Summary {
    errors: Vec<(PathBuf, String)>,
    success: usize,
}

impl Summary {
    fn parse<P: AsRef<Path>>(&mut self, path: P) {
        let contents = read_to_string(&path).unwrap();
        let result = parse_class(&contents);
        if let Err(err) = result {
            self.errors.push((path.as_ref().to_path_buf(), err.to_string()));
        } else {
            self.success += 1;
        }
    }

    fn total(&self) -> usize {
        return self.errors.len() + self.success;
    }

    // TODO: impl display
    fn print(&self) {
        for (file, error) in &self.errors {
            println!("Error in file: {:?}", file);
            println!("{}", error);
        }
        println!();
        println!("Parsed {} files.", self.total());
        println!("\t valid: {}", self.success);
        println!("\terrors: {}", self.errors.len());
    
        if !self.errors.is_empty() {
            println!("\nSee above for errors.");
        }
    }
}

fn main() {
    let mut args = env::args().skip(1);
    let path = args.next().expect("path argument is required");
    let path = PathBuf::from(path);

    let mut summary = Summary {
        errors: Vec::new(),
        success: 0,
    };

    if path.is_dir() {
        for entry in read_dir(&path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() && path.extension() == Some("cls".as_ref()) {
                summary.parse(path);
            }
        }
    } else {
        summary.parse(path);
    }

    summary.print();
}
