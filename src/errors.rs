use std::fmt::Debug;

use crate::{ast::BinaryOperator, interpret::Value};

#[derive(Clone, PartialEq, Eq)]
pub enum Error {
    Parsing(String),
    Boolean(String),
    Operator(String),
    OperatorType(String, BinaryOperator),
    TypeError(Value, Value),
    UnexpectedType(Value, String),
    UndeclaredVar(String),
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Parsing(src) => write!(f, "Error in parsing instruction: {}", src),
            Error::Boolean(b) => write!(f, "Wrong boolean form: {}", b),
            Error::Operator(op) => write!(f, "Unknown operator, got {}", op),
            Error::OperatorType(typename, op) => {
                write!(f, "Unapplicable operator for {}: {:?}", typename, op)
            }
            Error::TypeError(t1, t2) => write!(
                f,
                "t1 and t2 should have the same type, got t1: {:?}, t2: {:?}",
                t1, t2
            ),
            Error::UnexpectedType(v, ty) => write!(f, "Expected {}, got {:?}", ty, v),
            Error::UndeclaredVar(v) => write!(f, "Unknown var: {}", v),
        }
    }
}
