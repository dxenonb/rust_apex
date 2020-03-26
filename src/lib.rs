use pest::Parser;
use pest::iterators::Pairs;
use pest::error::Error as PestError;

#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "apex-grammar.pest"]
struct ApexParser;

pub fn parse_class(input: &str) -> Result<Pairs<Rule>, PestError<Rule>> {
    ApexParser::parse(Rule::class, input.as_ref())
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