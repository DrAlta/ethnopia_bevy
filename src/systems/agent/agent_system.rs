use bevy::prelude::*;
use ethnolib::sandbox::{actions::{ActionResult, GotoRequest, UseOnRequest}, ai::StackItem};
use crate::systems::{agent::{handle_prayer, Agent, AgentState}, FindInInventoryRequest};

pub fn agent_system(
    mut query: Query<(Entity, &mut Agent)>,
    mut action_result: EventReader<ActionResult>,
    mut goto_requests: EventWriter<GotoRequest>,
    mut find_in_inventory_requests: EventWriter<FindInInventoryRequest>,
    mut use_on_requests: EventWriter<UseOnRequest>,
    mut commands: Commands,

) {

    for ActionResult { agent_id, action_id, result } in action_result.read(){
        let Ok((_, mut agent)) = query.get_mut(*agent_id) else {
            continue;
        };
        let AgentState::WaitForAction(action_waiting_for_id) = &agent.state else {
            continue;
        };

        if action_waiting_for_id == action_id {
            let result = match result {
                ethnolib::sandbox::actions::Result::Success => StackItem::success(),
                ethnolib::sandbox::actions::Result::Failure => StackItem::failure(),
            };
            agent.cpu.stack.push(result);
            agent.state = AgentState::Running;

            commands.entity(*agent_id).remove::<ActionResult>();
        }
    }


    for (agent_id , mut agent) in &mut query {
        let Agent { cpu, blackboard, bt , state} = agent.as_mut();
        match state {
            AgentState::Running => {
                match cpu.step(bt, blackboard) {
                    Ok(ok) => {
                        handle_prayer(
                            agent_id, 
                            ok, 
                            cpu, 
                            &mut goto_requests,
                            &mut find_in_inventory_requests,
                            &mut use_on_requests,
                            state);
                    },
                    Err(_) => todo!(),
                }
            }
            AgentState::WaitForAction(_action_id) => {
                continue
            },
        }
    }
}
