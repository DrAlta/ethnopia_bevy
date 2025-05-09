use crate::systems::agent::AgentState;
use bevy::prelude::*;
use ethnolib::sandbox::ai::{Blackboard, BlackboardValue, TaskPool, ThreadName, CPU};

type TaskPoolId = TaskPool;

#[derive(Debug, Component, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Agent {
    pub cpu: CPU,
    pub blackboard: Blackboard<String, BlackboardValue>,
    pub bt: TaskPoolId,
    pub state: AgentState,
    // main should accept Init for when the the agent is first started. 
    // On a restart TOS with be Succes if it exited with Successs and 
    // Failute if it exited with a Failure
    pub main: ThreadName,
}
impl Agent {
    pub fn new(
        main: ThreadName, 
        blackboard: Blackboard<String, BlackboardValue>,
        bt: TaskPoolId,
    ) -> Self {
        let cpu = CPU::load(main.clone());
        let state = AgentState::Running;
        Agent { 
            cpu, 
            blackboard,
            bt,
            state, 
            main,
        }
    }
}