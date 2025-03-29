use bevy::prelude::*;
use ethnolib::{sandbox::{actions::{ActionResult, GotoRequest}, ai::{Blackboard, BlackboardValue, StackItem, Status, TreePool, CPU}, world::Movement}, vec2, Number};
use std::hash::{DefaultHasher, Hash, Hasher};


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum AgentState{
    Running,
    WaitForAction(u64)
}

#[derive(Debug, Component, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Agent{
    pub cpu: CPU,
    pub blackboard: Blackboard<String, BlackboardValue>,
    pub bt: TreePool,
    pub state: AgentState,
}

pub fn agent_system(
    mut query: Query<(Entity, &mut Agent)>,
    mut goto_requests: EventWriter<GotoRequest>,
    mut action_result: EventReader<ActionResult>,
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
                        handle_prayer(agent_id, ok, cpu, &mut goto_requests, state);
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
fn handle_prayer(
    agent_id: Entity,
    ok: Status,
    cpu: &mut CPU,
    goto_requests: &mut EventWriter<GotoRequest>,
    state: &mut AgentState,
) -> (){
    match ok {
        Status::Success => todo!(),
        Status::Failure => todo!(),
        Status::FindInInventory { ../*item_class*/ } => todo!(),
        Status::UseOn(_, _) => todo!(),
        Status::FindNearest {../* x, y, item_class*/ } => todo!(),
        Status::GetEnergy(_entity) => todo!(),
        Status::GetLocation(_entity) => todo!(),
        Status::GetHp(_entity) => todo!(),
        Status::GetIsInventoryGE { ../*agent, item_class, amount*/ } => todo!(),
        Status::GetEntities { ../*min_x, min_y, max_x, max_y*/ } => todo!(),
        Status::RemoveEntitiesOfType(_item) => todo!(),
        Status::RetainEntitiesOfType(_item) => todo!(),
        Status::Running(inpulse_id) => {
            match inpulse_id{
                ethnolib::sandbox::ai::InpulseId::Act1 => todo!(),
                ethnolib::sandbox::ai::InpulseId::Act2 => todo!(),
                ethnolib::sandbox::ai::InpulseId::Act3 => todo!(),
                ethnolib::sandbox::ai::InpulseId::GoTo => {
                    if let Some(StackItem::Coord{ x, y }) = cpu.stack.pop() {
                        let movement = Movement{ 
                            target: vec2(
                                Into::<Number>::into(x),
                                Into::<Number>::into(y),
                            ),
                            speed: Number::FIVE 
                        };
                        let salt = 0;
                        let mut s = DefaultHasher::new();
                        salt.hash(&mut s);
                        "Goto".hash(&mut s);
                        agent_id.hash(&mut s);
                        movement.hash(&mut s);
                        let action_id = s.finish();
                        goto_requests.send(GotoRequest { 
                            action_id,
                            agent_id,
                            movement,
                        });
                        *state = AgentState::WaitForAction(action_id);
                    }
                },
                ethnolib::sandbox::ai::InpulseId::Plant => todo!(),
                ethnolib::sandbox::ai::InpulseId::Take => todo!(),
                ethnolib::sandbox::ai::InpulseId::Use => todo!(),
                ethnolib::sandbox::ai::InpulseId::EatClass(_food_class) => {
                    
                },
            }
        },
        Status::None => todo!(),
    }

}