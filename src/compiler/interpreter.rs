use crate::Compile;

pub struct Interpreter;

impl Compile for Interpreter {}

#[cfg(test)]
mod tests {
    #[test]
    fn basics() {
        assert_eq!(3, 3);
    }
}
