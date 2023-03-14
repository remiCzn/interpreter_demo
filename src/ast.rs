use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Times,
    Divides,
    And,
    Or,
    More,
    MoreOrEqual,
    Less,
    LessOrEqual,
    Equal,
    Different,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Node {
    Int(i32),
    NodeSeq(Vec<Node>),
    Bool(bool),
    BinaryExpr {
        op: BinaryOperator,
        lterm: Box<Node>,
        rterm: Box<Node>,
    },
    If {
        cond: Box<Node>,
        then_term: Box<Node>,
        else_term: Box<Node>,
    },
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Int(i) => write!(f, "{}", i),
            Node::Bool(b) => write!(f, "{}", b),
            Node::BinaryExpr { rterm, op, lterm } => write!(f, "{} {:?} {}", lterm, op, rterm),
            Node::NodeSeq(list) => {
                let mut ss = String::new();
                for inst in list {
                    ss.push_str(format!("{}", inst).as_str());
                }
                write!(f, "{}", ss)
            }
            Node::If {
                cond,
                then_term,
                else_term,
            } => write!(
                f,
                "if({}) {{ {} }} else {{ {} }}",
                cond, then_term, else_term
            ),
        }
    }
}
