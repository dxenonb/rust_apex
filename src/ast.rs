// Scratch pad for AST layout, not yet compiled

pub struct File {
    comments:
    item: Item,
}

pub enum Item {
    Trigger(Trigger),
    Class(Class),
}

pub struct Trigger;

pub struct Class {
    members: Vec<Member>,
}

pub enum Member {
    Field(Field),
    InnerClass(Class),
    Method(Method),
}

// expression extraction - this builds and runs!

#[derive(Debug)]
enum BinOp {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Assign(Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
enum Expr {
    Primary(String),
    BinOp(Box<BinOp>),
}

pub fn apply_prec(input: &str) {
    let pairs = ApexParser::parse(Rule::test_input, input.as_ref());

    let mut pairs = match pairs {
        Ok(pairs) => pairs,
        Err(err) => {
            panic!("{}", err);
        },
    };

    println!("Parsed: {:?}", &pairs);

    let prec = PrecClimber::new(vec![
        Operator::new(Rule::test_eq, Assoc::Right),
        Operator::new(Rule::test_add, Assoc::Left) | Operator::new(Rule::test_sub, Assoc::Left),
        Operator::new(Rule::test_mul, Assoc::Left),
    ]);

    let expr = pairs.next().unwrap();
    let expr = extract_expr(&prec, expr.into_inner());
    println!("{:?}", expr);
}

fn extract_expr<'s, P>(
    prec: &PrecClimber<Rule>,
    pairs: P,
) -> Expr
where P: Iterator<Item=Pair<'s, Rule>> {
    prec.climb(pairs, 
        |p| {
            match p.as_rule() {
                Rule::test_group => extract_expr(prec, peel_to_expr_pairs(p).unwrap()),
                Rule::ident => Expr::Primary(p.as_str().to_string()),
                _ => unreachable!(format!("got rule {:?}", p)),
            }
        },
        |lhs, op, rhs| {
            let lhs = Box::new(lhs);
            let rhs = Box::new(rhs);
            Expr::BinOp(Box::new(match op.as_rule() {
                Rule::test_add => BinOp::Add(lhs, rhs),
                Rule::test_sub => BinOp::Sub(lhs, rhs),
                Rule::test_mul => BinOp::Mul(lhs, rhs),
                Rule::test_eq => BinOp::Assign(lhs, rhs),
                _ => unreachable!(),
            }))
        },
    )
}

fn peel_to_expr_pairs(pair: Pair<Rule>) -> Option<Pairs<Rule>> {
    match pair.as_rule() {
        Rule::test_group => Some(
            pair.into_inner()
                .next()?
                .into_inner()
        ),
        _ => None,
    }
}
