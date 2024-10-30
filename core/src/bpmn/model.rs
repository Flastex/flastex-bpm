// This file is part of Flastex BPM, an AGPLv3 licensed project.
// See the LICENSE.md file at the root of the repository for details.

pub mod connecting_objects;
/// Model Layer
/// Contains the static configuration
/// of the task (e.g., Task ID, name, type of task).
///
/// BPMN 2.0 primarily defines:
/// - Flow Objects: Events, Activities (Tasks, Sub-processes), Gateways.
/// - Connecting Objects: Sequence Flows, Message Flows, Associations.
/// - Swimlanes: Pools and Lanes.
/// - Artifacts: Data Objects, Groups, Annotations.
pub mod errors;
pub mod flow_objects;
pub mod process;
pub mod script;
pub mod xml_parser;
