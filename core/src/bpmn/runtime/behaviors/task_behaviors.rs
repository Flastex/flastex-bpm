use crate::bpmn::runtime::execution_context::ExecutionContext;

pub trait TaskBehavior {
    fn execute(&self, context: &mut ExecutionContext);
}

pub struct SimpleTaskBehavior;

impl TaskBehavior for SimpleTaskBehavior {
    fn execute(&self, context: &mut ExecutionContext) {
        println!("Executing task: {}", context.token.current_node_id);
        // After executing, move the token to the next node
        context.move_token("next_node_id"); // You would dynamically figure out the next node
    }
}
