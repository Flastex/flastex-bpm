// This file is part of Flastex BPM, an AGPLv3 licensed project.
// See the LICENSE.md file at the root of the repository for details.

use std::any::Any;

use crate::bpmn::model::connecting_objects::sequence_flows::SequenceFlowId;

use super::FlowObjectBehavior;

/// Enum representing different types of gateways.
#[derive(Clone, strum::Display, strum::AsRefStr, strum::EnumDiscriminants, PartialEq, Debug)]
#[strum_discriminants(derive(strum::EnumString), name(Type), strum(ascii_case_insensitive))]
pub enum GatewayType {
    ExclusiveGateway(ExclusiveGateway),
    ParallelGateway(ParallelGateway),
    ComplexGateway(ComplexGateway),
    EventBasedGateway(EventBasedGateway),
    InclusiveGateway(InclusiveGateway),
}

impl Type {
    pub fn to_event_type<T: GatewayBehavior>(self, gateway: T) -> GatewayType {
        match self {
            Type::ExclusiveGateway => GatewayType::ExclusiveGateway(
                gateway
                    .as_any()
                    .downcast_ref::<ExclusiveGateway>()
                    .expect("Failed to downcast to ExclusiveGateway")
                    .clone(),
            ),
            Type::ParallelGateway => GatewayType::ParallelGateway(
                gateway
                    .as_any()
                    .downcast_ref::<ParallelGateway>()
                    .expect("Failed to downcast to ParallelGateway")
                    .clone(),
            ),
            Type::ComplexGateway => GatewayType::ComplexGateway(
                gateway
                    .as_any()
                    .downcast_ref::<ComplexGateway>()
                    .expect("Failed to downcast to ComplexGateway")
                    .clone(),
            ),
            Type::EventBasedGateway => GatewayType::EventBasedGateway(
                gateway
                    .as_any()
                    .downcast_ref::<EventBasedGateway>()
                    .expect("Failed to downcast to EventBasedGateway")
                    .clone(),
            ),
            Type::InclusiveGateway => GatewayType::InclusiveGateway(
                gateway
                    .as_any()
                    .downcast_ref::<InclusiveGateway>()
                    .expect("Failed to downcast to InclusiveGateway")
                    .clone(),
            ),
        }
    }
}

/// Trait that represents the common behavior for all BPMN gateways.
pub trait GatewayBehavior: FlowObjectBehavior {
    /// Returns the type of the gateway (e.g., Exclusive, Parallel, etc.).
    fn gateway_type(&self) -> Type;
    /// Returns the direction of the gateway (e.g., Unspecified, Converging, Diverging).
    fn direction(&self) -> &GatewayDirection;
    /// Returns the default flow of the gateway, if specified.
    fn default_flow(&self) -> Option<&SequenceFlowId>;
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
    pub name: String,
    pub direction: GatewayDirection,
    pub default_flow: Option<SequenceFlowId>,
}

impl ExclusiveGateway {
    /// Creates a new exclusive gateway.
    pub fn new(
        name: &str,
        direction: GatewayDirection,
        default_flow: Option<SequenceFlowId>,
    ) -> Self {
        ExclusiveGateway {
            name: name.to_string(),
            direction,
            default_flow,
        }
    }
}

/// Implementation of the `FlowObjectBehavior` trait for `ExclusiveGateway`.
impl FlowObjectBehavior for ExclusiveGateway {
    fn flow_object_type(&self) -> super::Type {
        super::Type::Gateway
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Implementation of the `GatewayBehavior` trait for `ExclusiveGateway`.
impl GatewayBehavior for ExclusiveGateway {
    fn gateway_type(&self) -> Type {
        Type::ExclusiveGateway
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
    pub name: String,
    pub direction: GatewayDirection,
}

impl ParallelGateway {
    /// Creates a new parallel gateway.
    pub fn new(name: &str, direction: GatewayDirection) -> Self {
        ParallelGateway {
            name: name.to_string(),
            direction,
        }
    }
}

/// Implementation of the `FlowObjectBehavior` trait for `ParallelGateway`.
impl FlowObjectBehavior for ParallelGateway {
    fn flow_object_type(&self) -> super::Type {
        super::Type::Gateway
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Implementation of the `GatewayBehavior` trait for `ParallelGateway`.
impl GatewayBehavior for ParallelGateway {
    fn gateway_type(&self) -> Type {
        Type::ParallelGateway
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
    pub name: String,
    pub direction: GatewayDirection,
    pub default_flow: Option<SequenceFlowId>,
}

impl InclusiveGateway {
    /// Creates a new inclusive gateway.
    pub fn new(
        name: &str,
        direction: GatewayDirection,
        default_flow: Option<SequenceFlowId>,
    ) -> Self {
        InclusiveGateway {
            name: name.to_string(),
            direction,
            default_flow,
        }
    }
}

/// Implementation of the `FlowObjectBehavior` trait for `InclusiveGateway`.
impl FlowObjectBehavior for InclusiveGateway {
    fn flow_object_type(&self) -> super::Type {
        super::Type::Gateway
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Implementation of the `GatewayBehavior` trait for `InclusiveGateway`.
impl GatewayBehavior for InclusiveGateway {
    fn gateway_type(&self) -> Type {
        Type::InclusiveGateway
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
    pub name: String,
    pub direction: GatewayDirection,
    pub activation_conditions: Vec<String>,
}

impl ComplexGateway {
    /// Creates a new complex gateway.
    pub fn new(
        name: &str,
        direction: GatewayDirection,
        activation_conditions: Vec<String>,
    ) -> Self {
        ComplexGateway {
            name: name.to_string(),
            direction,
            activation_conditions,
        }
    }
}

/// Implementation of the `FlowObjectBehavior` trait for `ComplexGateway`.
impl FlowObjectBehavior for ComplexGateway {
    fn flow_object_type(&self) -> super::Type {
        super::Type::Gateway
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Implementation of the `GatewayBehavior` trait for `ComplexGateway`.
impl GatewayBehavior for ComplexGateway {
    fn gateway_type(&self) -> Type {
        Type::ComplexGateway
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
    pub name: String,
    pub direction: GatewayDirection,
    pub event_gateway_type: EventBasedGatewayType,
    pub instantiate: bool,
}

impl EventBasedGateway {
    /// Creates a new event-based gateway.
    pub fn new(
        name: &str,
        direction: GatewayDirection,
        event_gateway_type: EventBasedGatewayType,
        instantiate: bool,
    ) -> Self {
        EventBasedGateway {
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
    fn flow_object_type(&self) -> super::Type {
        super::Type::Gateway
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Implementation of the `GatewayBehavior` trait for `EventBasedGateway`.
impl GatewayBehavior for EventBasedGateway {
    fn gateway_type(&self) -> Type {
        Type::EventBasedGateway
    }

    fn direction(&self) -> &GatewayDirection {
        &self.direction
    }

    fn default_flow(&self) -> Option<&SequenceFlowId> {
        None // EventBasedGateway doesn't have a default flow
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exclusive_gateway_creation() {
        let name = "Exclusive Gateway";
        let direction = GatewayDirection::Diverging;
        let default_flow = Some("flow_1".to_string());

        let gateway =
            ExclusiveGateway::new(name, direction.clone(), default_flow.clone());

        assert_eq!(gateway.name, name);
        assert_eq!(gateway.direction, direction);
        assert_eq!(gateway.default_flow, default_flow);
    }

    #[test]
    fn test_parallel_gateway_creation() {
        let name = "Parallel Gateway";
        let direction = GatewayDirection::Converging;

        let gateway = ParallelGateway::new(name, direction.clone());

        assert_eq!(gateway.name, name);
        assert_eq!(gateway.direction, direction);
    }

    #[test]
    fn test_inclusive_gateway_creation() {
        let name = "Inclusive Gateway";
        let direction = GatewayDirection::Mixed;
        let default_flow = Some("flow_2".to_string());

        let gateway =
            InclusiveGateway::new(name, direction.clone(), default_flow.clone());

        assert_eq!(gateway.name, name);
        assert_eq!(gateway.direction, direction);
        assert_eq!(gateway.default_flow, default_flow);
    }

    #[test]
    fn test_complex_gateway_creation() {
        let name = "Complex Gateway";
        let direction = GatewayDirection::Unspecified;
        let activation_conditions = vec!["condition_1".to_string(), "condition_2".to_string()];

        let gateway = ComplexGateway::new(
            name,
            direction.clone(),
            activation_conditions.clone(),
        );

        assert_eq!(gateway.name, name);
        assert_eq!(gateway.direction, direction);
        assert_eq!(gateway.activation_conditions, activation_conditions);
    }

    #[test]
    fn test_event_based_gateway_creation() {
        let name = "Event-Based Gateway";
        let direction = GatewayDirection::Diverging;
        let event_gateway_type = EventBasedGatewayType::Exclusive;
        let instantiate = true;

        let gateway = EventBasedGateway::new(
            name,
            direction.clone(),
            event_gateway_type.clone(),
            instantiate,
        );

        assert_eq!(gateway.name, name);
        assert_eq!(gateway.direction, direction);
        assert_eq!(gateway.event_gateway_type, event_gateway_type);
        assert_eq!(gateway.instantiate, instantiate);
    }

    #[test]
    fn test_exclusive_gateway_behavior() {
        let name = "Exclusive Gateway";
        let direction = GatewayDirection::Diverging;
        let default_flow = Some("flow_3".to_string());

        let gateway =
            ExclusiveGateway::new(name, direction.clone(), default_flow.clone());

        assert_eq!(gateway.gateway_type(), Type::ExclusiveGateway);
        assert_eq!(gateway.direction(), &direction);
        assert_eq!(gateway.default_flow(), default_flow.as_ref());
    }

    #[test]
    fn test_parallel_gateway_behavior() {
        let name = "Parallel Gateway";
        let direction = GatewayDirection::Converging;

        let gateway = ParallelGateway::new(name, direction.clone());

        assert_eq!(gateway.gateway_type(), Type::ParallelGateway);
        assert_eq!(gateway.direction(), &direction);
        assert_eq!(gateway.default_flow(), None);
    }

    #[test]
    fn test_inclusive_gateway_behavior() {
        let name = "Inclusive Gateway";
        let direction = GatewayDirection::Mixed;
        let default_flow = Some("flow_4".to_string());

        let gateway =
            InclusiveGateway::new(name, direction.clone(), default_flow.clone());

        assert_eq!(gateway.gateway_type(), Type::InclusiveGateway);
        assert_eq!(gateway.direction(), &direction);
        assert_eq!(gateway.default_flow(), default_flow.as_ref());
    }

    #[test]
    fn test_complex_gateway_behavior() {
        let name = "Complex Gateway";
        let direction = GatewayDirection::Unspecified;
        let activation_conditions = vec!["condition_3".to_string(), "condition_4".to_string()];

        let gateway = ComplexGateway::new(
            name,
            direction.clone(),
            activation_conditions.clone(),
        );

        assert_eq!(gateway.gateway_type(), Type::ComplexGateway);
        assert_eq!(gateway.direction(), &direction);
        assert_eq!(gateway.default_flow(), None);
    }

    #[test]
    fn test_event_based_gateway_behavior() {
        let name = "Event-Based Gateway";
        let direction = GatewayDirection::Diverging;
        let event_gateway_type = EventBasedGatewayType::Parallel;
        let instantiate = false;

        let gateway = EventBasedGateway::new(
            name,
            direction.clone(),
            event_gateway_type.clone(),
            instantiate,
        );

        assert_eq!(gateway.gateway_type(), Type::EventBasedGateway);
        assert_eq!(gateway.direction(), &direction);
        assert_eq!(gateway.default_flow(), None);
    }

    #[test]
    fn test_exclusive_gateway_as_any() {
        let name = "Exclusive Gateway";
        let direction = GatewayDirection::Diverging;
        let default_flow = Some("flow_5".to_string());

        let gateway =
            ExclusiveGateway::new(name, direction.clone(), default_flow.clone());

        let any_ref: &dyn Any = gateway.as_any();
        assert!(any_ref.is::<ExclusiveGateway>());
    }

    #[test]
    fn test_parallel_gateway_as_any() {
        let name = "Parallel Gateway";
        let direction = GatewayDirection::Converging;

        let gateway = ParallelGateway::new(name, direction.clone());

        let any_ref: &dyn Any = gateway.as_any();
        assert!(any_ref.is::<ParallelGateway>());
    }

    #[test]
    fn test_inclusive_gateway_as_any() {
        let name = "Inclusive Gateway";
        let direction = GatewayDirection::Mixed;
        let default_flow = Some("flow_6".to_string());

        let gateway =
            InclusiveGateway::new(name, direction.clone(), default_flow.clone());

        let any_ref: &dyn Any = gateway.as_any();
        assert!(any_ref.is::<InclusiveGateway>());
    }

    #[test]
    fn test_complex_gateway_as_any() {
        let name = "Complex Gateway";
        let direction = GatewayDirection::Unspecified;
        let activation_conditions = vec!["condition_5".to_string(), "condition_6".to_string()];

        let gateway = ComplexGateway::new(
            name,
            direction.clone(),
            activation_conditions.clone(),
        );

        let any_ref: &dyn Any = gateway.as_any();
        assert!(any_ref.is::<ComplexGateway>());
    }

    #[test]
    fn test_event_based_gateway_as_any() {
        let name = "Event-Based Gateway";
        let direction = GatewayDirection::Diverging;
        let event_gateway_type = EventBasedGatewayType::Exclusive;
        let instantiate = true;

        let gateway = EventBasedGateway::new(
            name,
            direction.clone(),
            event_gateway_type.clone(),
            instantiate,
        );

        let any_ref: &dyn Any = gateway.as_any();
        assert!(any_ref.is::<EventBasedGateway>());
    }
}
