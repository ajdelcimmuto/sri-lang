use crate::ast::operators::Operator;

#[derive(Debug)]
pub enum Node {
    Int(i32),
    Func { op: Operator, child: Box<Vec<Node>> },
}
