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

    macro_rules! parse {
        ($rule:ident, $input:expr) => {{
            let r = ApexParser::parse(Rule::$rule, $input);
            match r {
                Err(err) => {
                    panic!("Failed to parse \"{}\":\n\t{}", $input, err);
                }
                Ok(r) => r,
            }
        }};
    }

    macro_rules! expect_invalid {
        ($rule:ident, $input:expr) => {
            let r = ApexParser::parse(Rule::$rule, $input);
            match r {
                Ok(pairs) => {
                    panic!("Parser accepted \"{}\" as:\n\t{:?}", $input, pairs);
                }
                _ => {}
            }
        };
    }

    // TODO: extends, implements, nested classes, for loops, try/catch

    // TODO: Upgrade these to use the new "matches!" macro

    #[test]
    fn parses_noops() {
        parse!(block, "{}");
        parse!(block, "{;}");
        parse!(block, "{;;}");
        parse!(statement, ";");
        parse!(statement, "{ ;;; }");
    }

    #[test]
    fn parses_conditionals() {
        parse!(statement_if, "if (true) ;");
        parse!(statement_if, "if (true) {}");
        parse!(statement_if, "if (true) { ;;; }");
        parse!(statement, "if (true) { ;;; }");
    }

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
    fn parses_func_args() {
        ApexParser::parse(Rule::func_args, "(x, 1000, 'ninja stars', null)").unwrap();
    }

    #[test]
    fn parses_type_names() {
        ApexParser::parse(Rule::type_name, "Integer").unwrap();
        ApexParser::parse(Rule::type_name, "List<Object>").unwrap();
        ApexParser::parse(Rule::type_name, "Map<Integer, List<Foo__c>>").unwrap();
    }

    #[test]
    fn parses_type_constructors() {
        ApexParser::parse(Rule::expr_unary, "new Account()").unwrap();
        ApexParser::parse(Rule::expr_unary, "new Account(x = 'string')").unwrap();
        ApexParser::parse(Rule::expr_unary, "new Account(x = 'string', y = 4)").unwrap();
        ApexParser::parse(Rule::expr_unary, "new Account(x=true)").unwrap();
        ApexParser::parse(Rule::expr_unary, "new List<Contact> {}").unwrap();
        ApexParser::parse(Rule::expr_unary, "new List<Contact> { 'apple', 'banana' }").unwrap();
    }

    #[test]
    fn parses_array_syntax() {
        parse!(type_name, "String[]");
        parse!(type_name, "List<Foo__c>[]");
        parse!(statement, "List<Foo__c>[] x;");
        parse!(statement, "String[] x = new String[] { 'a', 'b', 'c' };");
    }

    #[test]
    fn parses_strings() {
        ApexParser::parse(Rule::string, "''").unwrap();
        ApexParser::parse(Rule::string, "'foo'").unwrap();
        ApexParser::parse(Rule::string, "'\\tf\'o\'o'").unwrap();
    }

    #[test]
    fn parses_declarations() {
        let code = "Map<String, Object> data = (Map<String, Object>) JSON.deserializeUntyped(res.getBody());";
        parse!(statement, code);
    }

    #[test]
    fn parses_new_exprs() {
        ApexParser::parse(
            Rule::expr_unary,
            "new Map<Integer, List<Foo__c>>(alpha, beta)"
        ).unwrap();
        ApexParser::parse(
            Rule::expr_unary,
            "new Map<Integer, List<Foo__c>>(alpha)"
        ).unwrap();
    }

    #[test]
    fn parses_chained_el_ops() {
        // can't actually happen in Apex? but it probably shouldn't be a parser error
        ApexParser::parse(
            Rule::expr,
            "a.b()()()"
        ).unwrap();

        ApexParser::parse(
            Rule::expr,
            "a.b()[0]()"
        ).unwrap();
        ApexParser::parse(
            Rule::expr,
            "a[0][1]"
        ).unwrap();
    }

    #[test]
    fn parses_chained_method_calls() {
        ApexParser::parse(
            Rule::expr,
            "a.b().y.x().z.y()"
        ).unwrap();
    }

    #[test]
    fn parses_multiple_unary_ops() {
        ApexParser::parse(
            Rule::expr,
            "!!----4"
        ).unwrap();
    }

    #[test]
    #[should_panic]
    fn disallows_trailing_commas() {
        ApexParser::parse(
            Rule::expr_unary,
            "new Map<Integer, List<Foo__c>>(
                alpha,
                beta,
            )"
        ).unwrap();
    }

    #[test]
    fn parses_type_casts() {
        parse!(type_cast, "(Integer)");
        parse!(type_cast, "(Map<Integer, Integer>)");
    }

    #[test]
    fn parses_unary_expr() {
        parse!(expr_unary, "++ (Integer) !-apple --");
    }

    #[test]
    fn parses_properties() {
        parse!(class_item, "public static Integer field { get () {} }");
        parse!(class_item, "public static Integer field { get {} }");
        parse!(class_item, "public static Integer field { get; }");
        parse!(class_item, "public static Integer field { get ( ) { } }");
    }

    #[test]
    fn parses_ident_with_kw_prefix() {
        parse!(statement, "foo.put(nullStartsWithKeyword);");
    }
}
