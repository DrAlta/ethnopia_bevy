use crate::systems::agent::AgentState;
use bevy::prelude::*;
use ethnolib::sandbox::{
    EntityId,
    ai::{Blackboard, BlackboardValue, CPU, TaskPool, ThreadName},
};

type TaskPoolId = TaskPool;

#[derive(Debug, Component, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Agent {
    pub cpu: CPU,
    pub blackboard: Blackboard<String, BlackboardValue>,
    pub task_pool: TaskPoolId,
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
        task_pool: TaskPoolId,
    ) -> Self {
        let cpu = CPU::load(main.clone());
        let state = AgentState::Running;
        Agent {
            cpu,
            blackboard,
            task_pool,
            state,
            main,
        }
    }
    pub fn add(
        main: ThreadName,
        task_pool: TaskPoolId,
        agent_id: EntityId,
        commands: &mut Commands,
    ) {
        commands.entity(agent_id).insert(Self::new(
            main,
            Blackboard::from(
                [(
                    "self".to_owned(),
                    Into::<BlackboardValue>::into(agent_id).into(),
                )]
                .into(),
            ),
            task_pool,
        ));
    }
}
