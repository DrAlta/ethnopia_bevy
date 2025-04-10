use crate::systems::agent::AgentState;
use bevy::prelude::*;
use ethnolib::sandbox::ai::{Blackboard, BlackboardValue, CPU, TaskPool};

#[derive(Debug, Component, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Agent {
    pub cpu: CPU,
    pub blackboard: Blackboard<String, BlackboardValue>,
    pub bt: TaskPool,
    pub state: AgentState,
}
