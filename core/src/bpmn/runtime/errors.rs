use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct PathIdentifierParseError {
    pub message: String,
}

impl fmt::Display for PathIdentifierParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PathIdentifier string parse error: {}", self.message)
    }
}

// Define the main error for BPMN parsing
#[derive(Debug)]
pub enum ProcessEngineError {
    PathIdentifierFromStringError(PathIdentifierParseError),
}

impl fmt::Display for ProcessEngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessEngineError::PathIdentifierFromStringError(e) => {
                write!(f, "PathIdentifier string parse error: {}", e.message)
            }
        }
    }
}
