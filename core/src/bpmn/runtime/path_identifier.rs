// This file is part of Flastex BPM, an AGPLv3 licensed project.
// See the LICENSE.md file at the root of the repository for details.

use std::fmt;

use crate::bpmn::model::{flow_objects::FlowObjectId, process::ProcessId};

use super::process_instance::ProcessInstanceId;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct LoopIdentifier {
    loop_index: usize,
}

impl LoopIdentifier {
    pub fn new(loop_index: usize) -> Self {
        Self { loop_index }
    }

    pub fn loop_index(&self) -> usize {
        self.loop_index
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PathElement {
    process_id: ProcessId,
    flow_object_id: FlowObjectId,
    process_instance_id: ProcessInstanceId,
    loop_id: Option<LoopIdentifier>,
}

impl PathElement {
    pub fn new(
        process_id: ProcessId,
        flow_object_id: FlowObjectId,
        process_instance_id: ProcessInstanceId,
        loop_id: Option<LoopIdentifier>,
    ) -> Self {
        Self {
            process_id,
            flow_object_id,
            process_instance_id,
            loop_id,
        }
    }

    pub fn process_id(&self) -> &ProcessId {
        &self.process_id
    }

    pub fn flow_object_id(&self) -> &FlowObjectId {
        &self.flow_object_id
    }

    pub fn process_instance_id(&self) -> &ProcessInstanceId {
        &self.process_instance_id
    }

    pub fn loop_id(&self) -> Option<&LoopIdentifier> {
        self.loop_id.as_ref()
    }
}

/// A path identifier is a unique identifier for a path through a process model.
/// It is a sequence of path elements, each of which is a tuple of a process ID,
/// a flow object ID, and a loop index.
/// The loop index is optional, as not all flow objects are part of a loop.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PathIdentifier {
    /// The ID of the process the token is part of.
    pub path_elements: Vec<PathElement>,
}

impl PathIdentifier {
    pub fn new(path_elements: Vec<PathElement>) -> Self {
        Self { path_elements }
    }

    pub fn path_elements(&self) -> &Vec<PathElement> {
        &self.path_elements
    }
}

impl fmt::Display for PathIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut path_string = String::new();
        for path_element in &self.path_elements {
            path_string.push_str(&format!(
                "({}-{}-{})",
                path_element.process_id(),
                path_element.flow_object_id(),
                path_element.loop_id().map(|l| l.loop_index()).unwrap_or(0)
            ));
            path_string.push_str("->");
        }
        write!(f, "{}", path_string)
    }
}
