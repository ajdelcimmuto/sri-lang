use pest::{self, Parser};

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"] // relative to src
struct SriParser;

pub fn parse(source: &str) {
    let parse_result = SriParser::parse(Rule::Func, source).unwrap();
    let tokens = parse_result.tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}
