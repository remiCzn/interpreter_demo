use crate::ast::Node::{Bool, Int};
use crate::ast::{BinaryOperator, Node};
use crate::errors::Error;
use pest::iterators::Pair;
use pest::{self, Parser};

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct Parse;

pub fn parse(source: &str) -> Result<Vec<Node>, Error> {
    use crate::errors::Error::Parsing;
    let pairs = if let Ok(pairs) = Parse::parse(Rule::Program, source) {
        pairs
    } else {
        return Err(Parsing(source.to_string()));
    };
    let mut res: Vec<Node> = vec![];
    //Parse instructions one by one;
    for pair in pairs {
        if let Rule::ExprList = pair.as_rule() {
            for inst in pair.into_inner() {
                match parse_exprlist(inst) {
                    Ok(t) => res.push(t),
                    Err(e) => return Err(e),
                }
            }
        }
    }
    Ok(res)
}

fn parse_exprlist(pair: Pair<Rule>) -> Result<Node, Error> {
    match pair.as_rule() {
        Rule::ExprList => {
            let mut instructions: Vec<Node> = vec![];
            for instr in pair.into_inner() {
                match parse_exprlist(instr) {
                    Ok(res) => instructions.push(res),
                    Err(e) => return Err(e),
                }
            }
            Ok(Node::NodeSeq(instructions))
        }
        Rule::Int => {
            let istr = pair.as_str();
            let int: i32 = istr.parse().unwrap();
            Ok(Int(int))
        }
        Rule::Bool => match pair.as_str() {
            "True" => Ok(Bool(true)),
            "False" => Ok(Bool(false)),
            a => Err(Error::Boolean(a.to_string())),
        },
        Rule::If => {
            let mut terms = pair.into_inner();
            match parse_exprlist(terms.next().unwrap()) {
                Ok(cond) => match parse_exprlist(terms.next().unwrap()) {
                    Ok(then_term) => match parse_exprlist(terms.next().unwrap()) {
                        Ok(else_term) => Ok(Node::If {
                            cond: Box::from(cond),
                            then_term: Box::from(then_term),
                            else_term: Box::from(else_term),
                        }),
                        Err(e) => Err(e),
                    },
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            }
        }
        Rule::BinaryExpr => {
            let mut terms = pair.into_inner();
            match parse_exprlist(terms.next().unwrap()) {
                Ok(t1) => {
                    let op_str = terms.next().unwrap().as_str();
                    match parse_operator(op_str) {
                        Some(op) => match parse_exprlist(terms.next().unwrap()) {
                            Ok(t2) => Ok(Node::BinaryExpr {
                                op,
                                lterm: Box::from(t1),
                                rterm: Box::from(t2),
                            }),
                            Err(e) => Err(e),
                        },
                        None => Err(Error::Operator(op_str.to_string())),
                    }
                }
                Err(e) => Err(e),
            }
        }
        Rule::Let => {
            let mut terms = pair.into_inner();
            let var_name = terms.next().unwrap().as_str();
            match parse_exprlist(terms.next().unwrap()) {
                Ok(t) => Ok(Node::Let(var_name.to_string(), Box::from(t))),
                Err(e) => Err(e),
            }
        }
        Rule::Var => Ok(Node::Var(pair.as_str().to_string())),
        Rule::Function => Ok(Node::Bool(false)),
        _ => Err(Error::Parsing(pair.as_str().to_string())),
    }
}

fn parse_operator(op: &str) -> Option<BinaryOperator> {
    match op {
        "+" => Some(BinaryOperator::Plus),
        "-" => Some(BinaryOperator::Minus),
        "*" => Some(BinaryOperator::Times),
        "/" => Some(BinaryOperator::Divides),
        "<" => Some(BinaryOperator::Less),
        "<=" => Some(BinaryOperator::LessOrEqual),
        ">" => Some(BinaryOperator::More),
        ">=" => Some(BinaryOperator::MoreOrEqual),
        "||" => Some(BinaryOperator::Or),
        "&&" => Some(BinaryOperator::And),
        "==" => Some(BinaryOperator::Equal),
        "!=" => Some(BinaryOperator::Different),
        _ => None,
    }
}
