use pest::Parser;

#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "apex-grammar.pest"] // relative to src
struct ApexParser;

fn main() {
    let pairs = match ApexParser::parse(Rule::class, "public class Foo { }") {
        Ok(r) => r,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };
    for pair in pairs {
        println!("Pair: {:?}", pair);
    }
}
