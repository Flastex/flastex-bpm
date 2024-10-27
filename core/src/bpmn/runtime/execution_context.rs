use std::collections::HashMap;

use super::token::Token;

#[derive(Clone, Debug)]
pub struct ExecutionContext {
    pub token: Token,
    pub variables: HashMap<String, String>, // To track execution variables
}

impl ExecutionContext {
    pub fn move_token(&mut self, new_node_id: &str) {
        self.token.current_node_id = new_node_id.to_string();
    }
}
