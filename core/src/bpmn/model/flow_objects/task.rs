// This file is part of Flastex BPM, an AGPLv3 licensed project.
// See the LICENSE.md file at the root of the repository for details.

use std::any::Any;

use super::{activity::ActivityBehavior, FlowObjectBehavior};

#[derive(Clone, strum::Display, strum::AsRefStr, strum::EnumDiscriminants, PartialEq, Debug)]
#[strum_discriminants(name(Type))]
pub enum TaskType {
    // these will be different Task structs
    UserTask(Task),
    ServiceTask(Task),
    ScriptTask(Task),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Task {
    pub name: String,
    task_type: Type,
}

impl Task {
    pub fn new(name: &str, task_type: Type) -> Self {
        Task {
            name: name.to_string(),
            task_type: task_type,
        }
    }

    pub fn task_type(&self) -> &Type {
        &self.task_type
    }
}

impl ActivityBehavior for Task {

    fn name(&self) -> &str {
        &self.name
    }
    
    fn activity_type(&self) -> super::activity::Type {
        super::activity::Type::Task
    }
}

impl FlowObjectBehavior for Task {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn flow_object_type(&self) -> super::Type {
        super::Type::Activity
    }
}


pub fn is_automatic_task(task_type: &TaskType) -> bool {
    match task_type {
        TaskType::ServiceTask(_) => true,
        TaskType::ScriptTask(_) => true,
        _ => false,        
    }
}
