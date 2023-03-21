use std::collections::HashMap;

use crate::ast::{BinaryOperator, Node};
use crate::errors::Error;
use crate::parse;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Value {
    Int(i32),
    Bool(bool),
    Null,
}

type Env = HashMap<String, Value>;

pub fn interpret(node: Node, env: Env) -> Result<(Value, Env), Error> {
    use crate::interpret::Value::{Bool, Int, Null};
    match node {
        Node::NodeSeq(list) => {
            let n = list.len();
            // Execute the first instructions
            if n >= 2 {
                for noden in list[0..(n - 2)].iter().cloned() {
                    let _ = interpret(noden, env.clone());
                }
            }
            // Return the last one
            if let Some(inst) = list.get(n - 1) {
                let a = inst;
                interpret(a.clone(), env)
            } else {
                Ok((Null, env))
            }
        }
        Node::Int(n) => Ok((Int(n), env)),
        Node::BinaryExpr { op, lterm, rterm } => {
            let t1 = match interpret(*lterm, env.clone()) {
                Ok((t1, _)) => t1,
                Err(e) => return Err(e),
            };

            let t2 = match interpret(*rterm, env.clone()) {
                Ok((t2, _)) => t2,
                Err(e) => return Err(e),
            };

            let value = match (t1, t2) {
                (Int(a), Int(b)) => match op {
                    BinaryOperator::Plus => Ok(Int(a + b)),
                    BinaryOperator::Minus => Ok(Int(a - b)),
                    BinaryOperator::Times => Ok(Int(a * b)),
                    BinaryOperator::Divides => Ok(Int(a / b)),
                    BinaryOperator::Different => Ok(Bool(a != b)),
                    BinaryOperator::Equal => Ok(Bool(a == b)),
                    BinaryOperator::More => Ok(Bool(a > b)),
                    BinaryOperator::MoreOrEqual => Ok(Bool(a >= b)),
                    BinaryOperator::Less => Ok(Bool(a < b)),
                    BinaryOperator::LessOrEqual => Ok(Bool(a <= b)),
                    _ => Err(Error::OperatorType("Int".to_string(), op)),
                },
                (Bool(a), Bool(b)) => match op {
                    BinaryOperator::And => Ok(Bool(a && b)),
                    BinaryOperator::Or => Ok(Bool(a || b)),
                    BinaryOperator::Different => Ok(Bool(a != b)),
                    BinaryOperator::Equal => Ok(Bool(a == b)),
                    _ => Err(Error::OperatorType("Bool".to_string(), op)),
                },
                (t1, t2) => Err(Error::TypeError(t1, t2)),
            };
            match value {
                Ok(v) => Ok((v, env)),
                Err(e) => Err(e),
            }
        }
        Node::Bool(b) => Ok((Bool(b), env)),
        Node::If {
            cond,
            then_term,
            else_term,
        } => {
            let cond = match interpret(*cond, env.clone()) {
                Ok((b, _)) => b,
                Err(e) => return Err(e),
            };
            if let Bool(res) = cond {
                if res {
                    interpret(*then_term, env)
                } else {
                    interpret(*else_term, env)
                }
            } else {
                Err(Error::UnexpectedType(cond, "Bool".to_string()))
            }
        }
        Node::Let(name, value) => {
            let value = match interpret(*value, env.clone()) {
                Ok((b, _)) => b,
                Err(e) => return Err(e),
            };
            let mut env = env;
            env.insert(name.trim().to_string(), value);
            Ok((Null, env))
        }
        Node::Var(name) => {
            let name = name.trim().to_string();
            if let Some(value) = env.get(&name) {
                Ok((value.clone(), env))
            } else {
                Err(Error::UndeclaredVar(name.to_string()))
            }
        }
    }
}

pub fn run(source: &str) -> Result<Value, Error> {
    let parsed = parse(source);
    match parsed {
        Ok(parsed) => {
            println!("AST: {:?}", parsed);
            let mut env = HashMap::new();
            let mut value = Value::Null;
            for p in parsed {
                match interpret(p, env) {
                    Ok((v, e)) => {
                        value = v;
                        env = e;
                    }
                    Err(e) => return Err(e),
                }
            }
            println!("Environnement: {:?}", env);
            Ok(value)
        }
        Err(e) => Err(e),
    }
}
