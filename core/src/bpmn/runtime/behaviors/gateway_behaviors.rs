use crate::bpmn::runtime::execution_context::ExecutionContext;

pub trait GatewayBehavior {
    fn route(&self, context: &mut ExecutionContext);
}

pub struct ExclusiveGatewayBehavior;

impl GatewayBehavior for ExclusiveGatewayBehavior {
    fn route(&self, context: &mut ExecutionContext) {
        // Route the token based on a condition
        let condition = context.variables.get("condition_key").unwrap(); // Use process variables
        if condition == "condition_1" {
            context.move_token("path_1");
        } else {
            context.move_token("path_2");
        }
    }
}

pub struct ParallelGatewayBehavior;

impl GatewayBehavior for ParallelGatewayBehavior {
    fn route(&self, context: &mut ExecutionContext) {
        // Spawn multiple tokens for parallel execution
        context.move_token("path_1");
        let mut new_token = context.clone(); // Create a new token for parallel execution
        new_token.move_token("path_2");
        println!("Token split for parallel execution");
    }
}
