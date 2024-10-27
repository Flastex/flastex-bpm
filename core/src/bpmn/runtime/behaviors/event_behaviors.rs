use crate::bpmn::runtime::execution_context::ExecutionContext;

pub trait EventBehavior {
    fn execute(&self, context: &mut ExecutionContext);
}

pub struct StartEventBehavior;

impl EventBehavior for StartEventBehavior {
    fn execute(&self, context: &mut ExecutionContext) {
        println!("Starting the process at: {}", context.token.current_node_id);
        context.move_token("task1"); // Start the process by moving to the first task
    }
}