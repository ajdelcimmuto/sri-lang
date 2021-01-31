use crate::ast::node::Node;
use crate::ast::operators::Operator;
use pest::{self, Parser};

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"] // relative to src
struct SriParser;

/**
 * source - source file as a str
**/
pub fn parse(source: &str) -> std::result::Result<Vec<Node>, pest::error::Error<Rule>> {
    {
        let mut ast = vec![];
        let parse_result = SriParser::parse(Rule::Program, source)?;
        for pair in parse_result {
            if let Rule::Func = pair.as_rule() {
                ast.push(build_ast_from_func(pair));
            }
        }
        return Ok(ast);
    }

    pub fn build_ast_from_func(pair: pest::iterators::Pair<Rule>) -> Node {
        match pair.as_rule() {
            Rule::Func => {
                let mut pair = pair.into_inner();
                let op = pair.next().unwrap().as_str();
                println!("{:?}", op);
                let mut ints = Vec::new();
                for int in pair {
                    match int.as_rule() {
                        Rule::Int => {
                            let istr = int.as_str();
                            let int: i32 = istr.parse().unwrap();
                            println!("{}", int);
                            ints.push(Node::Int(int));
                        }
                        _ => unreachable!(),
                    }
                }

                parse_func(op, ints)
            }
            _ => unreachable!(),
        }
    }

    pub fn parse_func(pair: &str, ints: Vec<Node>) -> Node {
        println!("BRUH: {:?}", ints);
        Node::Func {
            op: match pair {
                "sum" => Operator::sum,
                "sub" => Operator::sub,
                _ => unreachable!(),
            },
            child: Box::new(ints),
        }
    }
}
