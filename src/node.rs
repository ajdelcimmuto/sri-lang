pub enum Node {
    Int(i32),
    func { op: Operator, child: Box<Node> },
}
