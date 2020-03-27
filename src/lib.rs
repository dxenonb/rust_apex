use pest::Parser;
use pest::iterators::Pairs;
use pest::error::Error as PestError;

#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "apex-grammar.pest"]
struct ApexParser;

pub fn parse_class(input: &str) -> Result<Pairs<Rule>, PestError<Rule>> {
    ApexParser::parse(Rule::class_file, input.as_ref())
}

#[cfg(test)]
mod test {

    use super::*;

    // TODO: Upgrade these to use the new "matches!" macro

    #[test]
    fn parses_integers() {
        ApexParser::parse(Rule::num, "124").unwrap();
    }

    #[test]
    fn parses_annotations() {
        ApexParser::parse(Rule::annotation, "@isTest").unwrap();
        ApexParser::parse(Rule::annotation, "@IsTest").unwrap();
        ApexParser::parse(Rule::annotation, "@RemoteAction").unwrap();
        ApexParser::parse(Rule::annotation, "@Arg(x = 2)").unwrap();
    }

    #[test]
    fn parses_type_names() {
        ApexParser::parse(Rule::type_name, "Integer").unwrap();
        ApexParser::parse(Rule::type_name, "List<Object>").unwrap();
        ApexParser::parse(Rule::type_name, "Map<Integer, List<Foo__c>>").unwrap();
    }
}