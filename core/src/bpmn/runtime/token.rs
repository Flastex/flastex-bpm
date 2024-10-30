// This file is part of Flastex BPM, an AGPLv3 licensed project.
// See the LICENSE.md file at the root of the repository for details.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Token {
    // Unique token ID
    pub id: String,
    // The ID of the current node the token is at
    pub current_node_id: String,
    // The current state of the token
    pub state: TokenState,
    // Optional reference to the parent token (for subprocesses)
    pub parent_token_id: Option<String>,
    // Tokens spawned by this token (for subprocesses or parallel executions)
    pub child_tokens: Vec<Token>,
    // Process variables (key-value pairs) that affect flow
    pub process_variables: HashMap<String, String>,
}

#[derive(Clone, strum::Display, strum::EnumString, PartialEq, Debug)]
pub enum TokenState {
    // The token is moving through the process
    Active,
    // The token is paused at a task waiting for input or external action
    Paused,
    // The token has reached the end of the process
    Completed,
    // The token has been consumed by a gateway or end event
    Consumed,
    // The token was terminated early (e.g., due to process cancellation)
    Terminated,
}
