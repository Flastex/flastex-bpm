// This file is part of Flastex BPM, an AGPLv3 licensed project.
// See the LICENSE.md file at the root of the repository for details.

use std::collections::HashMap;

use super::{connecting_objects::sequence_flows::SequenceFlow, flow_objects::FlowObject};

#[derive(Clone, strum::Display, strum::EnumString, PartialEq, Debug)]
pub enum ProcessError {
    FlowObjectAlreadyExists(String),
    FlowObjectNotFound(String),
    SequenceFlowNotFound(String),
}

impl std::error::Error for ProcessError {}

/// Type aliases for readability
type ProcessRoles = Vec<String>;
type Properties = Vec<String>;
type LaneSets = Vec<String>;
type Artifacts = Vec<String>;
type ResourceRoles = Vec<String>;
type CorrelationSubscriptions = Vec<String>;
type Supports = Vec<String>;
type FlowObjects = HashMap<String, FlowObject>;
type SequenceFlows = Vec<SequenceFlow>;

/// Enum to describe the executability of a process.
#[derive(Clone, strum::Display, strum::EnumString, PartialEq, Debug)]
pub enum ExecutableStatus {
    Executable,
    NonExecutable,
}

/// Enum to describe whether a process is open or closed.
#[derive(Clone, strum::Display, strum::EnumString, PartialEq, Debug)]
pub enum ProcessState {
    Open,
    Closed,
}

/// Enum for process type.
#[derive(Clone, strum::Display, strum::EnumString, PartialEq, Debug)]
pub enum ProcessType {
    /// Private Business Processes are those internal to a specific organization.
    /// There are two types of private Processes: executable and non-executable.
    /// Executable private Processes are those that are intended to be executed.
    /// Non-executable private Processes is a Process that has been modeled for
    /// the purpose of documenting Process behavior at a modeler-defined level of detail.
    Private,
    /// A public Process represents the interactions between a private
    /// Business Process and another Process or Participant
    Public,
}

impl Default for ProcessType {
    fn default() -> Self {
        ProcessType::Private
    }
}

/// Enum to represent optional collaboration reference.
#[derive(Clone, strum::Display, strum::EnumString, PartialEq, Debug)]
pub enum CollaborationRef {
    Defined(String),
    Undefined,
}

/// Enum to represent optional auditing information.
#[derive(Clone, strum::Display, strum::EnumString, PartialEq, Debug)]
pub enum AuditingInfo {
    Present(String),
    Absent,
}

/// Enum to represent optional monitoring information.
#[derive(Clone, strum::Display, strum::EnumString, PartialEq, Debug)]
pub enum MonitoringInfo {
    Enabled(String),
    Disabled,
}

/// The `Process` struct represents a BPMN process with various types and attributes.
#[derive(Clone, Debug)]
pub struct Process {
    id: String,
    name: String,
    process_type: ProcessType,
    executable_status: ExecutableStatus,
    process_state: ProcessState,
    definitional_collaboration_ref: CollaborationRef,
    auditing: AuditingInfo,
    monitoring: MonitoringInfo,
    process_roles: ProcessRoles,
    properties: Properties,
    lane_sets: LaneSets,
    artifacts: Artifacts,
    resource_roles: ResourceRoles,
    correlation_subscriptions: CorrelationSubscriptions,
    supports: Supports,
    flow_objects: FlowObjects,
    sequence_flows: SequenceFlows,
}

impl Process {
    // Constructor
    /// Creates a new `Process` instance with default values.
    pub fn new() -> Self {
        Process::default()
    }

    // Getters and setters using custom types instead of bool and Option<T>

    /// Returns the id.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Sets the id.
    pub fn set_id(&mut self, id: &str) -> &mut Self {
        self.id = String::from(id);
        self
    }

    /// Returns the name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the name
    pub fn set_name(&mut self, name: &str) -> &mut Self {
        self.name = String::from(name);
        self
    }

    /// Returns the process type.
    pub fn process_type(&self) -> &ProcessType {
        &self.process_type
    }

    /// Sets the process type.
    pub fn set_process_type(&mut self, process_type: ProcessType) -> &mut Self {
        self.process_type = process_type;
        self
    }

    /// Returns whether the process is executable.
    pub fn executable_status(&self) -> &ExecutableStatus {
        &self.executable_status
    }

    /// Sets whether the process is executable.
    pub fn set_executable_status(&mut self, status: ExecutableStatus) -> &mut Self {
        self.executable_status = status;
        self
    }

    /// Returns whether the process is open or closed.
    pub fn process_state(&self) -> &ProcessState {
        &self.process_state
    }

    /// Sets whether the process is open or closed.
    pub fn set_process_state(&mut self, state: ProcessState) -> &mut Self {
        self.process_state = state;
        self
    }

    // Setter and getter for definitional_collaboration_ref
    /// Returns the definitional collaboration reference.
    pub fn definitional_collaboration_ref(&self) -> &CollaborationRef {
        &self.definitional_collaboration_ref
    }

    /// Sets the definitional collaboration reference.
    pub fn set_definitional_collaboration_ref(&mut self, ref_value: CollaborationRef) -> &mut Self {
        self.definitional_collaboration_ref = ref_value;
        self
    }

    // Setter and getter for auditing
    /// Returns the auditing information.
    pub fn auditing(&self) -> &AuditingInfo {
        &self.auditing
    }

    /// Sets the auditing information.
    pub fn set_auditing(&mut self, auditing: AuditingInfo) -> &mut Self {
        self.auditing = auditing;
        self
    }

    // Setter and getter for monitoring
    /// Returns the monitoring information.
    pub fn monitoring(&self) -> &MonitoringInfo {
        &self.monitoring
    }

    /// Sets the monitoring information.
    pub fn set_monitoring(&mut self, monitoring: MonitoringInfo) -> &mut Self {
        self.monitoring = monitoring;
        self
    }

    // Methods to manipulate process_role
    pub fn add_process_role(&mut self, role: String) -> &mut Self {
        self.process_roles.push(role);
        self
    }

    pub fn remove_process_role(&mut self, role: &str) -> &mut Self {
        self.process_roles.retain(|r| r != role);
        self
    }

    // Methods to manipulate properties, lane_sets, artifacts, etc., using type aliases
    pub fn add_property(&mut self, property: String) -> &mut Self {
        self.properties.push(property);
        self
    }

    pub fn remove_property(&mut self, property: &str) -> &mut Self {
        self.properties.retain(|p| p != property);
        self
    }

    pub fn add_lane_set(&mut self, lane: String) -> &mut Self {
        self.lane_sets.push(lane);
        self
    }

    pub fn remove_lane_set(&mut self, lane: &str) -> &mut Self {
        self.lane_sets.retain(|l| l != lane);
        self
    }

    pub fn add_artifact(&mut self, artifact: String) -> &mut Self {
        self.artifacts.push(artifact);
        self
    }

    pub fn remove_artifact(&mut self, artifact: &str) -> &mut Self {
        self.artifacts.retain(|a| a != artifact);
        self
    }

    pub fn add_resource_role(&mut self, role: String) -> &mut Self {
        self.resource_roles.push(role);
        self
    }

    pub fn remove_resource_role(&mut self, role: &str) -> &mut Self {
        self.resource_roles.retain(|r| r != role);
        self
    }

    pub fn add_correlation_subscription(&mut self, subscription: String) -> &mut Self {
        self.correlation_subscriptions.push(subscription);
        self
    }

    pub fn remove_correlation_subscription(&mut self, subscription: &str) -> &mut Self {
        self.correlation_subscriptions.retain(|c| c != subscription);
        self
    }

    pub fn add_support(&mut self, support: String) -> &mut Self {
        self.supports.push(support);
        self
    }

    pub fn remove_support(&mut self, support: &str) -> &mut Self {
        self.supports.retain(|s| s != support);
        self
    }

    // FlowObject methods
    pub fn flow_objects(&self) -> &FlowObjects {
        &self.flow_objects
    }

    pub fn add_flow_object(&mut self, flowobject: FlowObject) -> Result<&mut Self, ProcessError> {
        if self.flow_objects.contains_key(&flowobject.id) {
            return Err(ProcessError::FlowObjectAlreadyExists(flowobject.id.clone()));
        }
        self.flow_objects.insert(flowobject.id.clone(), flowobject);
        Ok(self)
    }

    pub fn remove_flow_object(&mut self, flowobject_id: &str) -> Result<&mut Self, ProcessError> {
        if self.flow_objects.remove(flowobject_id).is_none() {
            return Err(ProcessError::FlowObjectNotFound(flowobject_id.to_string()));
        }
        Ok(self)
    }

    // SequenceFlow methods
    pub fn sequence_flows(&self) -> &SequenceFlows {
        &self.sequence_flows
    }

    pub fn add_sequence_flow(&mut self, sequence_flow: SequenceFlow) -> &mut Self {
        self.sequence_flows.push(sequence_flow);
        self
    }

    pub fn remove_sequence_flow(
        &mut self,
        sequence_flow_id: &str,
    ) -> Result<&mut Self, ProcessError> {
        let index = self
            .sequence_flows
            .iter()
            .position(|sf| sf.id() == sequence_flow_id)
            .ok_or_else(|| ProcessError::SequenceFlowNotFound(sequence_flow_id.to_string()))?;
        self.sequence_flows.remove(index);
        Ok(self)
    }
}

impl Default for Process {
    /// Initializes the default values for `Process`.
    fn default() -> Self {
        Process {
            id: String::default(),
            name: String::default(),
            process_type: ProcessType::default(),
            executable_status: ExecutableStatus::NonExecutable,
            process_state: ProcessState::Open,
            definitional_collaboration_ref: CollaborationRef::Undefined,
            auditing: AuditingInfo::Absent,
            monitoring: MonitoringInfo::Disabled,
            process_roles: vec![],
            properties: vec![],
            lane_sets: vec![],
            artifacts: vec![],
            resource_roles: vec![],
            correlation_subscriptions: vec![],
            supports: vec![],
            flow_objects: HashMap::new(),
            sequence_flows: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::bpmn::model::flow_objects::{
        event::{self, Event, EventType},
        FlowObjectType,
    };

    use super::*;

    #[test]
    fn test_process_default() {
        let process = Process::default();
        assert_eq!(process.id(), "");
        assert_eq!(process.name(), "");
        assert_eq!(process.process_type(), &ProcessType::Private);
        assert_eq!(
            process.executable_status(),
            &ExecutableStatus::NonExecutable
        );
        assert_eq!(process.process_state(), &ProcessState::Open);
        assert_eq!(
            process.definitional_collaboration_ref(),
            &CollaborationRef::Undefined
        );
        assert_eq!(process.auditing(), &AuditingInfo::Absent);
        assert_eq!(process.monitoring(), &MonitoringInfo::Disabled);
        assert!(process.process_roles.is_empty());
        assert!(process.properties.is_empty());
        assert!(process.lane_sets.is_empty());
        assert!(process.artifacts.is_empty());
        assert!(process.resource_roles.is_empty());
        assert!(process.correlation_subscriptions.is_empty());
        assert!(process.supports.is_empty());
        assert!(process.flow_objects.is_empty());
        assert!(process.sequence_flows.is_empty());
    }

    #[test]
    fn test_setters_and_getters() {
        let mut process = Process::new();
        process
            .set_id("process_1")
            .set_name("Test Process")
            .set_process_type(ProcessType::Public)
            .set_executable_status(ExecutableStatus::Executable)
            .set_process_state(ProcessState::Closed)
            .set_definitional_collaboration_ref(CollaborationRef::Defined("collab_1".to_string()))
            .set_auditing(AuditingInfo::Present("audit_1".to_string()))
            .set_monitoring(MonitoringInfo::Enabled("monitor_1".to_string()));

        assert_eq!(process.id(), "process_1");
        assert_eq!(process.name(), "Test Process");
        assert_eq!(process.process_type(), &ProcessType::Public);
        assert_eq!(process.executable_status(), &ExecutableStatus::Executable);
        assert_eq!(process.process_state(), &ProcessState::Closed);
        assert_eq!(
            process.definitional_collaboration_ref(),
            &CollaborationRef::Defined("collab_1".to_string())
        );
        assert_eq!(
            process.auditing(),
            &AuditingInfo::Present("audit_1".to_string())
        );
        assert_eq!(
            process.monitoring(),
            &MonitoringInfo::Enabled("monitor_1".to_string())
        );
    }

    #[test]
    fn test_add_and_remove_process_role() {
        let mut process = Process::new();
        process.add_process_role("role_1".to_string());
        assert_eq!(process.process_roles.len(), 1);
        assert_eq!(process.process_roles[0], "role_1");

        process.remove_process_role("role_1");
        assert!(process.process_roles.is_empty());
    }

    #[test]
    fn test_add_and_remove_property() {
        let mut process = Process::new();
        process.add_property("property_1".to_string());
        assert_eq!(process.properties.len(), 1);
        assert_eq!(process.properties[0], "property_1");

        process.remove_property("property_1");
        assert!(process.properties.is_empty());
    }

    #[test]
    fn test_add_and_remove_lane_set() {
        let mut process = Process::new();
        process.add_lane_set("lane_1".to_string());
        assert_eq!(process.lane_sets.len(), 1);
        assert_eq!(process.lane_sets[0], "lane_1");

        process.remove_lane_set("lane_1");
        assert!(process.lane_sets.is_empty());
    }

    #[test]
    fn test_add_and_remove_artifact() {
        let mut process = Process::new();
        process.add_artifact("artifact_1".to_string());
        assert_eq!(process.artifacts.len(), 1);
        assert_eq!(process.artifacts[0], "artifact_1");

        process.remove_artifact("artifact_1");
        assert!(process.artifacts.is_empty());
    }

    #[test]
    fn test_add_and_remove_resource_role() {
        let mut process = Process::new();
        process.add_resource_role("resource_role_1".to_string());
        assert_eq!(process.resource_roles.len(), 1);
        assert_eq!(process.resource_roles[0], "resource_role_1");

        process.remove_resource_role("resource_role_1");
        assert!(process.resource_roles.is_empty());
    }

    #[test]
    fn test_add_and_remove_correlation_subscription() {
        let mut process = Process::new();
        process.add_correlation_subscription("subscription_1".to_string());
        assert_eq!(process.correlation_subscriptions.len(), 1);
        assert_eq!(process.correlation_subscriptions[0], "subscription_1");

        process.remove_correlation_subscription("subscription_1");
        assert!(process.correlation_subscriptions.is_empty());
    }

    #[test]
    fn test_add_and_remove_support() {
        let mut process = Process::new();
        process.add_support("support_1".to_string());
        assert_eq!(process.supports.len(), 1);
        assert_eq!(process.supports[0], "support_1");

        process.remove_support("support_1");
        assert!(process.supports.is_empty());
    }

    #[test]
    fn test_add_and_remove_flow_object() {
        let mut process = Process::new();
        let flow_object = FlowObject {
            id: "flow_1".to_string(),
            flow_object_type: FlowObjectType::Event(EventType::StartEvent(Event::new(
                "start",
                event::Type::StartEvent,
            ))),
        };
        process.add_flow_object(flow_object.clone()).unwrap();
        assert_eq!(process.flow_objects.len(), 1);
        assert_eq!(process.flow_objects.get("flow_1").unwrap(), &flow_object);

        process.remove_flow_object("flow_1").unwrap();
        assert!(process.flow_objects.is_empty());
    }

    #[test]
    fn test_add_and_remove_sequence_flow() {
        let mut process = Process::new();
        let mut sequence_flow = SequenceFlow::new();
        sequence_flow.set_id("seq_1".to_string());

        process.add_sequence_flow(sequence_flow.clone());
        assert_eq!(process.sequence_flows.len(), 1);
        let first_sequence_flow = process.sequence_flows.get(0);
        assert!(first_sequence_flow.is_some());
        let stored_sequence_flow = first_sequence_flow.unwrap();
        assert_eq!(stored_sequence_flow.id(), sequence_flow.id());

        process.remove_sequence_flow("seq_1").unwrap();
        assert!(process.sequence_flows.is_empty());
    }
}
