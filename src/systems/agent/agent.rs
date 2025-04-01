use bevy::prelude::*;
use ethnolib::sandbox::ai::{Blackboard, BlackboardValue, TreePool, CPU};
use crate::systems::agent::AgentState;


#[derive(Debug, Component, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Agent{
    pub cpu: CPU,
    pub blackboard: Blackboard<String, BlackboardValue>,
    pub bt: TreePool,
    pub state: AgentState,
}