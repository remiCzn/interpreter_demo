use crate::ast::Node::{Bool, Int};
use crate::ast::{BinaryOperator, Node};
use pest::iterators::Pair;
use pest::{self, Parser};

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct Parse;

pub fn parse(source: &str) -> Result<Vec<Node>, String> {
    let pairs = Parse::parse(Rule::Program, source).unwrap();
    let mut res: Vec<Node> = vec![];
    for pair in pairs {
        if let Rule::ExprList = pair.as_rule() {
            for inst in pair.into_inner() {
                println!("Parsing instruction: {:?}", inst.as_str());
                match parse_exprlist(inst) {
                    Ok(t) => res.push(t),
                    Err(e) => return Err(e),
                }
            }
        }
    }
    Ok(res)
}

fn parse_exprlist(pair: Pair<Rule>) -> Result<Node, String> {
    match pair.as_rule() {
        Rule::ExprList => {
            let mut instructions: Vec<Node> = vec![];
            for instr in pair.into_inner() {
                match parse_exprlist(instr) {
                    Ok(res) => instructions.push(res),
                    Err(_) => return Err("Error in expression list".to_string()),
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
            a => Err(format!("Wrong boolean form: {}", a)),
        },
        Rule::If => {
            let mut terms = pair.into_inner();
            if let Ok(cond) = parse_exprlist(terms.next().unwrap()) {
                if let Ok(then_term) = parse_exprlist(terms.next().unwrap()) {
                    if let Ok(else_term) = parse_exprlist(terms.next().unwrap()) {
                        Ok(Node::If {
                            cond: Box::from(cond),
                            then_term: Box::from(then_term),
                            else_term: Box::from(else_term),
                        })
                    } else {
                        Err("".to_string())
                    }
                } else {
                    Err("".to_string())
                }
            } else {
                Err("".to_string())
            }
        }
        Rule::BinaryExpr => {
            let mut terms = pair.into_inner();
            if let Ok(t1) = parse_exprlist(terms.next().unwrap()) {
                let op = parse_operator(terms.next().unwrap().as_str());
                if let Ok(t2) = parse_exprlist(terms.next().unwrap()) {
                    Ok(Node::BinaryExpr {
                        op,
                        lterm: Box::from(t1),
                        rterm: Box::from(t2),
                    })
                } else {
                    Err("".to_string())
                }
            } else {
                Err("".to_string())
            }
        }
        Rule::Let => {
            let mut terms = pair.into_inner();
            let var_name = terms.next().unwrap().as_str();
            if let Ok(t) = parse_exprlist(terms.next().unwrap()) {
                Ok(Node::Let(var_name.to_string(), Box::from(t)))
            } else {
                Err("".to_string())
            }
        }
        Rule::Var => Ok(Node::Var(pair.as_str().to_string())),
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
