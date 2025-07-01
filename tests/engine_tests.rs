use orion_calculator_lib::engine::evaluate;

#[test]
fn test_addition() {
    assert_eq!(evaluate("1 + 1"), Ok(2.0));
}

#[test]
fn test_subtraction() {
    assert_eq!(evaluate("1 - 1"), Ok(0.0));
}

#[test]
fn test_multiplication() {
    assert_eq!(evaluate("2 * 2"), Ok(4.0));
}

#[test]
fn test_division() {
    assert_eq!(evaluate("4 / 2"), Ok(2.0));
}

#[test]
fn test_precedence() {
    assert_eq!(evaluate("2 + 2 * 2"), Ok(6.0));
}

#[test]
fn test_parentheses() {
    assert_eq!(evaluate("(2 + 2) * 2"), Ok(8.0));
}

#[test]
fn test_invalid_expression() {
    assert!(evaluate("1 +").is_err());
}
