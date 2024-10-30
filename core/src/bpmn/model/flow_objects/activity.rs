// This file is part of Flastex BPM, an AGPLv3 licensed project.
// See the LICENSE.md file at the root of the repository for details.

use super::{task::TaskType, FlowObjectBehavior};

#[derive(Clone, strum::Display, strum::AsRefStr, strum::EnumDiscriminants, PartialEq, Debug)]
#[strum_discriminants(derive(strum::EnumString), name(Type))]
pub enum ActivityType {
    Task(TaskType),
    // TODO: SubProcess(SubProcessType),
}

/// An Activity is a type of FlowObject that is tipically a Task or a SubProcess
pub trait ActivityBehavior: FlowObjectBehavior {
    fn name(&self) -> &str;

    fn activity_type(&self) -> Type;
}
