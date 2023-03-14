// use crate::ast::Node;
// use core::panicking::panic;
// use std::ops::Deref;
//
// #[derive(Debug, Eq, PartialEq)]
// enum Type {
//     Int,
//     Bool,
//     Null,
// }
//
// pub struct TypedNode {
//     node: Node,
//     _type: Type,
// }
//
// pub fn typer(node: Node) -> TypedNode {
//     let _type = match node.clone() {
//         Node::NodeList(list) => {
//             for inst in list {
//                 typer(*inst);
//             }
//             let last = list.last().unwrap().deref();
//             typer(last.clone())._type
//         }
//         Node::Int(_) => Type::Int,
//         Node::Bool(_) => Type::Bool,
//         Node::BinaryExpr { .. } => {}
//         Node::If {
//             cond,
//             then_term,
//             else_term,
//         } => {
//             let t_cond = typer(*cond);
//             if t_cond._type != Type::Bool {
//                 panic!("Error type: if condition must be boolean");
//             }
//             let t_then = typer(*then_term);
//             let t_else = typer(*else_term);
//             if t_then._type != t_else._type {
//                 panic!("Error type: then and else body must have the same type");
//             }
//         }
//     };
//     TypedNode { node, _type }
// }
