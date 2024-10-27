use std::any::Any;

use super::activity::Activity;
use super::{FlowObjectBehavior, FlowObjectId};

#[derive(Clone, strum::Display, strum::EnumString, PartialEq, Debug)]
pub enum TaskType {
    UserTask,
    ServiceTask,
    ScriptTask,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Task {
    pub id: FlowObjectId,
    pub name: String,
    pub task_type: TaskType,
}

impl Task {
    pub fn new(id: &FlowObjectId, name: &str, task_type: &TaskType) -> Self {
        Task {
            id: id.clone(),
            name: name.to_string(),
            task_type: task_type.clone(),
        }
    }
}

impl Activity for Task {
    fn id(&self) -> &FlowObjectId {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl FlowObjectBehavior for Task {
    fn r#type(&self) -> super::FlowObjectType {
        super::FlowObjectType::Task
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn is_automatic_task(task: &Task) -> bool {
    task.task_type == TaskType::ServiceTask || task.task_type == TaskType::ScriptTask
}
