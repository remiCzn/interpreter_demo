#[cfg(test)]
mod tests {
    use crate::interpret::{run, Value};

    #[test]
    fn simple_test() {
        assert_eq!(run("3"), Ok(Value::Int(3)));
        assert_eq!(run("((2))"), Ok(Value::Int(2)));
        assert_eq!(run("(26)"), Ok(Value::Int(26)));
    }

    #[test]
    fn binary_tests() {
        assert_eq!(run("3 -2"), Ok(Value::Int(1)));
        assert_eq!(run("4+ 3"), Ok(Value::Int(7)));
        assert_eq!(run("6  *7"), Ok(Value::Int(42)));
        assert_eq!(run(" 5/ 4"), Ok(Value::Int(1)));
        assert_eq!(run("(3-2)"), Ok(Value::Int(1)));
        assert_eq!(run("(5*4)-(60/3)"), Ok(Value::Int(0)));
    }
}
