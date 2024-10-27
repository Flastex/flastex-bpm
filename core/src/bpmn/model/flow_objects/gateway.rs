use std::any::Any;

use crate::bpmn::model::connecting_objects::sequence_flows::SequenceFlowId;

use super::{FlowObjectBehavior, FlowObjectId};

/// Trait that represents the common behavior for all BPMN gateways.
pub trait GatewayBehavior: FlowObjectBehavior {
    /// Returns the type of the gateway (e.g., Exclusive, Parallel, etc.).
    fn gateway_type(&self) -> GatewayType;
    /// Returns the direction of the gateway (e.g., Unspecified, Converging, Diverging).
    fn direction(&self) -> &GatewayDirection;
    /// Returns the default flow of the gateway, if specified.
    fn default_flow(&self) -> Option<&SequenceFlowId>;
}

/// Enum representing different types of gateways.
#[derive(Clone, strum::Display, strum::EnumString, PartialEq, Debug)]
#[strum(ascii_case_insensitive)]
pub enum GatewayType {
    ExclusiveGateway,
    ParallelGateway,
    ComplexGateway,
    EventBasedGateway,
    InclusiveGateway,
}

/// Enum representing the direction of flow through the gateway.
#[derive(Clone, strum::Display, strum::EnumString, PartialEq, Debug)]
pub enum GatewayDirection {
    Unspecified,
    Converging,
    Diverging,
    Mixed,
}

/// Struct representing an exclusive gateway.
#[derive(Clone, PartialEq, Debug)]
pub struct ExclusiveGateway {
    pub id: FlowObjectId,
    pub name: String,
    pub direction: GatewayDirection,
    pub default_flow: Option<SequenceFlowId>,
}

impl ExclusiveGateway {
    /// Creates a new exclusive gateway.
    pub fn new(
        id: FlowObjectId,
        name: &str,
        direction: GatewayDirection,
        default_flow: Option<SequenceFlowId>,
    ) -> Self {
        ExclusiveGateway {
            id: id,
            name: name.to_string(),
            direction,
            default_flow,
        }
    }
}

/// Implementation of the `FlowObjectBehavior` trait for `ExclusiveGateway`.
impl FlowObjectBehavior for ExclusiveGateway {
    fn r#type(&self) -> super::FlowObjectType {
        super::FlowObjectType::Gateway
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Implementation of the `GatewayBehavior` trait for `ExclusiveGateway`.
impl GatewayBehavior for ExclusiveGateway {
    fn gateway_type(&self) -> GatewayType {
        GatewayType::ExclusiveGateway
    }

    fn direction(&self) -> &GatewayDirection {
        &self.direction
    }

    fn default_flow(&self) -> Option<&SequenceFlowId> {
        self.default_flow.as_ref()
    }
}

/// Struct representing a parallel gateway.
#[derive(Clone, PartialEq, Debug)]
pub struct ParallelGateway {
    pub id: FlowObjectId,
    pub name: String,
    pub direction: GatewayDirection,
}

impl ParallelGateway {
    /// Creates a new parallel gateway.
    pub fn new(id: FlowObjectId, name: &str, direction: GatewayDirection) -> Self {
        ParallelGateway {
            id: id,
            name: name.to_string(),
            direction,
        }
    }
}

/// Implementation of the `FlowObjectBehavior` trait for `ParallelGateway`.
impl FlowObjectBehavior for ParallelGateway {
    fn r#type(&self) -> super::FlowObjectType {
        super::FlowObjectType::Gateway
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Implementation of the `GatewayBehavior` trait for `ParallelGateway`.
impl GatewayBehavior for ParallelGateway {
    fn gateway_type(&self) -> GatewayType {
        GatewayType::ParallelGateway
    }

    fn direction(&self) -> &GatewayDirection {
        &self.direction
    }

    fn default_flow(&self) -> Option<&SequenceFlowId> {
        None
    }
}

/// Struct representing an inclusive gateway.
#[derive(Clone, PartialEq, Debug)]
pub struct InclusiveGateway {
    pub id: FlowObjectId,
    pub name: String,
    pub direction: GatewayDirection,
    pub default_flow: Option<SequenceFlowId>,
}

impl InclusiveGateway {
    /// Creates a new inclusive gateway.
    pub fn new(
        id: FlowObjectId,
        name: &str,
        direction: GatewayDirection,
        default_flow: Option<SequenceFlowId>,
    ) -> Self {
        InclusiveGateway {
            id: id.to_string(),
            name: name.to_string(),
            direction,
            default_flow,
        }
    }
}

/// Implementation of the `FlowObjectBehavior` trait for `InclusiveGateway`.
impl FlowObjectBehavior for InclusiveGateway {
    fn r#type(&self) -> super::FlowObjectType {
        super::FlowObjectType::Gateway
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Implementation of the `GatewayBehavior` trait for `InclusiveGateway`.
impl GatewayBehavior for InclusiveGateway {
    fn gateway_type(&self) -> GatewayType {
        GatewayType::InclusiveGateway
    }

    fn direction(&self) -> &GatewayDirection {
        &self.direction
    }

    fn default_flow(&self) -> Option<&SequenceFlowId> {
        self.default_flow.as_ref()
    }
}

/// Struct representing a complex gateway.
#[derive(Clone, PartialEq, Debug)]
pub struct ComplexGateway {
    pub id: FlowObjectId,
    pub name: String,
    pub direction: GatewayDirection,
    pub activation_conditions: Vec<String>,
}

impl ComplexGateway {
    /// Creates a new complex gateway.
    pub fn new(
        id: FlowObjectId,
        name: &str,
        direction: GatewayDirection,
        activation_conditions: Vec<String>,
    ) -> Self {
        ComplexGateway {
            id: id.to_string(),
            name: name.to_string(),
            direction,
            activation_conditions,
        }
    }
}

/// Implementation of the `FlowObjectBehavior` trait for `ComplexGateway`.
impl FlowObjectBehavior for ComplexGateway {
    fn r#type(&self) -> super::FlowObjectType {
        super::FlowObjectType::Gateway
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Implementation of the `GatewayBehavior` trait for `ComplexGateway`.
impl GatewayBehavior for ComplexGateway {
    fn gateway_type(&self) -> GatewayType {
        GatewayType::ComplexGateway
    }

    fn direction(&self) -> &GatewayDirection {
        &self.direction
    }

    fn default_flow(&self) -> Option<&SequenceFlowId> {
        None // ComplexGateway doesn't have a default flow
    }
}

/// Struct representing an event-based gateway.
#[derive(Clone, PartialEq, Debug)]
pub struct EventBasedGateway {
    pub id: FlowObjectId,
    pub name: String,
    pub direction: GatewayDirection,
    pub event_gateway_type: EventBasedGatewayType,
    pub instantiate: bool,
}

impl EventBasedGateway {
    /// Creates a new event-based gateway.
    pub fn new(
        id: FlowObjectId,
        name: &str,
        direction: GatewayDirection,
        event_gateway_type: EventBasedGatewayType,
        instantiate: bool,
    ) -> Self {
        EventBasedGateway {
            id: id.to_string(),
            name: name.to_string(),
            direction,
            event_gateway_type,
            instantiate,
        }
    }
}

/// Enum representing the type of event-based gateway (exclusive or parallel).
#[derive(Clone, strum::Display, strum::EnumString, PartialEq, Debug)]
pub enum EventBasedGatewayType {
    Exclusive,
    Parallel,
}

/// Implementation of the `FlowObjectBehavior` trait for `EventBasedGateway`.
impl FlowObjectBehavior for EventBasedGateway {
    fn r#type(&self) -> super::FlowObjectType {
        super::FlowObjectType::Gateway
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Implementation of the `GatewayBehavior` trait for `EventBasedGateway`.
impl GatewayBehavior for EventBasedGateway {
    fn gateway_type(&self) -> GatewayType {
        GatewayType::EventBasedGateway
    }

    fn direction(&self) -> &GatewayDirection {
        &self.direction
    }

    fn default_flow(&self) -> Option<&SequenceFlowId> {
        None // EventBasedGateway doesn't have a default flow
    }
}
