#[cfg(test)]
mod tests {
    use crate::interpret::{run, Value};

    #[test]
    fn simple_test() {
        assert_eq!(run("3"), Value::Int(3));
        assert_eq!(run("((2))"), Value::Int(2));
        assert_eq!(run("(26)"), Value::Int(26));
    }

    #[test]
    fn binary_tests() {
        assert_eq!(run("3 -2"), Value::Int(1));
        assert_eq!(run("4+ 3"), Value::Int(7));
        assert_eq!(run("6  *7"), Value::Int(42));
        assert_eq!(run(" 5/ 4"), Value::Int(1));
        assert_eq!(run("(3-2)"), Value::Int(1));
        assert_eq!(run("(5*4)-(60/3)"), Value::Int(0));
    }
}
