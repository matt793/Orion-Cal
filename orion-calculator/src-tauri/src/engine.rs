pub fn evaluate(expression: &str) -> Result<f64, String> {
    match meval::eval_str(expression) {
        Ok(result) => Ok(result),
        Err(e) => Err(e.to_string()),
    }
}
