use std::fmt;

#[derive(Debug)]
pub struct EvaluatorError(String);

impl fmt::Display for EvaluatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl EvaluatorError {
    pub fn new(msg: String) -> Self {
        EvaluatorError(msg)
    }
}
