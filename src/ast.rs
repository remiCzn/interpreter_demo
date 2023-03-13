#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Times,
    Divides,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Node {
    NodeList(Vec<Box<Node>>),
    Int(i32),
    BinaryExpr {
        op: Operator,
        lterm: Box<Node>,
        rterm: Box<Node>,
    },
    Bool(bool),
    If {
        cond: Box<Node>,
        then_term: Box<Node>,
        else_term: Option<Box<Node>>,
    },
}
