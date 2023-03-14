use crate::ast::Node::{Bool, Int};
use crate::ast::{BinaryOperator, Node};
use pest::iterators::Pair;
use pest::{self, Parser};

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct Parse;

pub fn parse(source: &str) -> Vec<Node> {
    let pairs = Parse::parse(Rule::Program, source).unwrap();
    let mut res = vec![];
    for pair in pairs {
        if let Rule::ExprList = pair.as_rule() {
            for inst in pair.into_inner() {
                println!("Parsing instruction: {:?}", inst.as_str());
                res.push(parse_exprlist(inst));
            }
        }
    }
    res
}

fn parse_exprlist(pair: Pair<Rule>) -> Node {
    match pair.as_rule() {
        Rule::ExprList => {
            let mut instructions = vec![];
            for instr in pair.into_inner() {
                instructions.push(parse_exprlist(instr));
            }
            Node::NodeSeq(instructions)
        }
        Rule::Int => {
            let istr = pair.as_str();
            let int: i32 = istr.parse().unwrap();
            Int(int)
        }
        Rule::Bool => match pair.as_str() {
            "True" => Bool(true),
            "False" => Bool(false),
            a => panic!("Wrong boolean form: {}", a),
        },
        Rule::If => {
            let mut terms = pair.into_inner();
            let cond = Box::from(parse_exprlist(terms.next().unwrap()));
            let then_term = Box::from(parse_exprlist(terms.next().unwrap()));
            let else_term = Box::from(parse_exprlist(terms.next().unwrap()));
            Node::If {
                cond,
                then_term,
                else_term,
            }
        }
        Rule::BinaryExpr => {
            let mut terms = pair.into_inner();
            let t1 = parse_exprlist(terms.next().unwrap());
            let op = parse_operator(terms.next().unwrap().as_str());
            let t2 = parse_exprlist(terms.next().unwrap());
            Node::BinaryExpr {
                op,
                lterm: Box::from(t1),
                rterm: Box::from(t2),
            }
        }
        Rule::Let => {
            let mut terms = pair.into_inner();
            let var_name = terms.next().unwrap().as_str();
            Node::Let(
                var_name.to_string(),
                Box::from(parse_exprlist(terms.next().unwrap())),
            )
        }
        Rule::Var => Node::Var(pair.as_str().to_string()),
        _ => panic!("Can't parse this {:?}", pair),
    }
}

fn parse_operator(op: &str) -> BinaryOperator {
    match op {
        "+" => BinaryOperator::Plus,
        "-" => BinaryOperator::Minus,
        "*" => BinaryOperator::Times,
        "/" => BinaryOperator::Divides,
        "<" => BinaryOperator::Less,
        "<=" => BinaryOperator::LessOrEqual,
        ">" => BinaryOperator::More,
        ">=" => BinaryOperator::MoreOrEqual,
        "||" => BinaryOperator::Or,
        "&&" => BinaryOperator::And,
        "==" => BinaryOperator::Equal,
        "!=" => BinaryOperator::Different,
        u => panic!("Unknown operator: {}", u),
    }
}
