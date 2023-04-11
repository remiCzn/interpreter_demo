#[cfg(test)]
mod tests {
    use crate::interpret::{run, Value};

    #[test]
    fn simple_test() {
        assert_eq!(run("3"), Value::Int(3));
        assert_eq!(run("((2))"), Value::Int(2));
        assert_eq!(run("(26)"), Value::Int(26));
        assert!(matches!(run("(2))"), Value::Error(_)));
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

    #[test]
    fn boolean_test() {
        assert_eq!(run("3>2"), Value::Bool(true));
        assert_eq!(run("5<1"), Value::Bool(false));
        assert_eq!(run("(3+2) == 5"), Value::Bool(true));
        assert_eq!(run("(2+2) != 4"), Value::Bool(false));
    }

    #[test]
    fn type_test() {
        assert!(matches!(run("2+True"), Value::Error(_)));
        assert!(matches!(run("False -3"), Value::Error(_)));
        assert!(matches!(run("False>3"), Value::Error(_)));
        assert!(matches!(run("3 && 2"), Value::Error(_)));
        assert!(matches!(run("True || 24"), Value::Error(_)));
    }
}
