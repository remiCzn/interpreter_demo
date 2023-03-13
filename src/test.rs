#[cfg(test)]
mod tests {
    use crate::interpret::{run, Return};

    #[test]
    fn simple_test() {
        assert_eq!(run("3"), Return::Int(3));
        assert_eq!(run("((2))"), Return::Int(2));
        assert_eq!(run("(26)"), Return::Int(26));
    }

    #[test]
    fn binary_tests() {
        assert_eq!(run("3 -2"), Return::Int(1));
        assert_eq!(run("4+ 3"), Return::Int(7));
        assert_eq!(run("6  *7"), Return::Int(42));
        assert_eq!(run(" 5/ 4"), Return::Int(1));
        assert_eq!(run("(3-2)"), Return::Int(1));
        assert_eq!(run("(5*4)-(60/3)"), Return::Int(0));
    }
}
