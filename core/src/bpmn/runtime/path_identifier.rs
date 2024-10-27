use std::fmt;
use std::str::FromStr;

use super::errors::PathIdentifierParseError;

#[derive(Debug, Clone)]
pub struct LoopIdentifier {
    pub loop_index: usize,
}

// Define the PathIdentifier struct with a simple display trait
#[derive(Debug, Clone)]
pub struct PathIdentifier {
    // id of an element of a process
    pub flow_element_id: String,
    pub loop_id: Option<LoopIdentifier>,
    pub parent_token_id: Option<String>,
}

impl PathIdentifier {
    pub fn new_for_event(flow_element_id: String, parent_token_id: Option<String>) -> Self {
        PathIdentifier {
            flow_element_id,
            loop_id: None,
            parent_token_id,
        }
    }

    pub fn new_for_sequence_flow(flow_element_id: String, parent_token_id: Option<String>) -> Self {
        PathIdentifier {
            flow_element_id,
            loop_id: None,
            parent_token_id,
        }
    }

    pub fn new_for_flow_object(
        flow_element_id: String,
        loop_id: Option<LoopIdentifier>,
        parent_token_id: Option<String>,
    ) -> Self {
        PathIdentifier {
            flow_element_id,
            loop_id,
            parent_token_id,
        }
    }
}

impl fmt::Display for PathIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let loop_id_str = match &self.loop_id {
            Some(loop_id) => format!("/{}", loop_id.loop_index),
            None => "".to_string(),
        };
        let parent_token_id_str = match &self.parent_token_id {
            Some(parent_token_id) => format!("/{}", parent_token_id),
            None => "".to_string(),
        };
        write!(
            f,
            "{}{}{}",
            self.flow_element_id, loop_id_str, parent_token_id_str
        )
    }
}

impl FromStr for PathIdentifier {
    type Err = PathIdentifierParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split the string by '/'
        let mut parts = s.split('/').collect::<Vec<&str>>();

        if parts.is_empty() {
            return Err(PathIdentifierParseError {
                message: "Input string is empty.".to_string(),
            });
        }

        // The first part is the flow_element_id
        let flow_element_id = parts.remove(0).to_string();

        // Try parsing the loop index if it's present
        let loop_id = if let Some(loop_str) = parts.first() {
            if let Ok(loop_index) = loop_str.parse::<usize>() {
                parts.remove(0); // Remove loop_index part
                Some(LoopIdentifier { loop_index })
            } else {
                None
            }
        } else {
            None
        };

        // The remaining part is the parent_token_id (if present)
        let parent_token_id = parts.first().map(|&s| s.to_string());

        Ok(PathIdentifier {
            flow_element_id,
            loop_id,
            parent_token_id,
        })
    }
}
