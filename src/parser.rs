use crate::ast::Node::Int;
use crate::ast::{BoolOperator, Node, Operator};
use pest::iterators::Pair;
use pest::{self, Parser};

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct Parse;

pub fn parse(source: &str) -> Result<Node, String> {
    let pairs = Parse::parse(Rule::Program, source).unwrap();
    let mut res = None;
    for pair in pairs {
        if let Rule::ExprList = pair.as_rule() {
            for inst in pair.into_inner() {
                println!("Parsing instruction: {:?}", inst.as_str());
                res = Some(parse_exprlist(inst));
            }
        }
    }
    match res {
        None => Err(format!("Can't parse: {source}")),
        Some(a) => Ok(a),
    }
}

fn parse_exprlist(pair: Pair<Rule>) -> Node {
    match pair.as_rule() {
        Rule::ExprList => {
            let mut instructions = vec![];
            for instr in pair.into_inner() {
                instructions.push(Box::from(parse_exprlist(instr)));
            }
            Node::NodeList(instructions)
        }
        Rule::Int => {
            let istr = pair.as_str();
            let int: i32 = istr.parse().unwrap();
            Int(int)
        }
        Rule::IntBinaryExpr => {
            let mut terms = pair.into_inner();
            let lterm = parse_exprlist(terms.next().unwrap());
            let op = parse_ops(terms.next().unwrap().as_str());
            let rterm = parse_exprlist(terms.next().unwrap());
            Node::IntBinaryExpr {
                op,
                lterm: Box::from(lterm),
                rterm: Box::from(rterm),
            }
        }
        Rule::BoolBinaryExpr => {
            let mut terms = pair.into_inner();
            let lterm = parse_exprlist(terms.next().unwrap());
            let op = parse_bool_ops(terms.next().unwrap().as_str());
            let rterm = parse_exprlist(terms.next().unwrap());
            Node::BoolBinaryExpr {
                op,
                lterm: Box::from(lterm),
                rterm: Box::from(rterm),
            }
        }
        Rule::Bool => parse_bool(pair),
        Rule::If => {
            let mut terms = pair.into_inner();
            let cond = Box::from(parse_bool(terms.next().unwrap()));
            let then_term = Box::from(parse_exprlist(terms.next().unwrap()));
            let _else_term = terms.next();
            let else_term = match _else_term {
                None => None,
                Some(a) => Some(Box::from(parse_exprlist(a))),
            };
            Node::If {
                cond,
                then_term,
                else_term,
            }
        }
        _ => panic!("Can't parse this {:?}", pair),
    }
}

fn parse_ops(op_str: &str) -> Operator {
    match op_str {
        "+" => Operator::Plus,
        "-" => Operator::Minus,
        "*" => Operator::Times,
        "/" => Operator::Divides,
        a => panic!("Unexisting operator: {a}"),
    }
}

fn parse_bool_ops(op_str: &str) -> BoolOperator {
    match op_str {
        "&&" => BoolOperator::And,
        "||" => BoolOperator::Or,
        a => panic!("Unexisting operator: {a}"),
    }
}

fn parse_bool(bool: Pair<Rule>) -> Node {
    match bool.as_str() {
        "True" => Node::Bool(true),
        "False" => Node::Bool(false),
        _ => parse_exprlist(bool),
    }
}
