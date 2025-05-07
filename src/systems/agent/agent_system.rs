use crate::{systems::{
    actions::{self, ActionResult, DropRequest, GotoRequest, UseOnRequest},
    agent::{handle_prayer, Agent, AgentState},
    query::{
        FindInInventoryRequest, FindInInventoryResult, FindNearestRequest, FindNearestResult, GetEnergyRequest, GetEnergyResult, GetHpRequest, GetLocationRequest, GetLocationResult
    },
}, GameState};
use bevy::prelude::*;
use ethnolib::sandbox::{ai::StackItem, Location};


pub fn agent_system(
    mut query: Query<(Entity, &mut Agent)>,
    mut action_results: EventReader<ActionResult>,
    mut find_in_inventory_results: EventReader<FindInInventoryResult>,
    mut find_nearest_results: EventReader<FindNearestResult>,
    mut get_energy_result: EventReader<GetEnergyResult>,
    mut get_location_result: EventReader<GetLocationResult>,


    mut drop_request: EventWriter<DropRequest>,
    mut goto_request: EventWriter<GotoRequest>,
    mut find_in_inventory_request: EventWriter<FindInInventoryRequest>,
    mut find_nearest_request: EventWriter<FindNearestRequest>,
    mut get_energy_request: EventWriter<GetEnergyRequest>,
    mut get_hp_request: EventWriter<GetHpRequest>,
    mut get_location_request: EventWriter<GetLocationRequest>,
    mut use_on_request: EventWriter<UseOnRequest>,

    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    //mut commands: Commands,
) {
    let mut made_world_query = false;
    
    for ActionResult {
        agent_id,
        prayer_id,
        result,
    } in action_results.read()
    {
        let Ok((_, mut agent)) = query.get_mut(*agent_id) else {
            continue;
        };
        let AgentState::WaitForAction(action_waiting_for_id) = &agent.state else {
            continue;
        };

        if action_waiting_for_id == prayer_id {
            let result = match result {
                actions::Result::Success => StackItem::success(),
                actions::Result::Failure => StackItem::failure(),
            };
            agent.cpu.stack.push(result);
            agent.state = AgentState::Running;
        }
    }

    // handle FindInInventory prays being answered
    for FindInInventoryResult {
        agent_id,
        prayer_id,
        found_item_id_maybe,
    } in find_in_inventory_results.read()
    {
        let Ok((_, mut agent)) = query.get_mut(*agent_id) else {
            continue;
        };
        let AgentState::WaitForAction(action_waiting_for_id) = &agent.state else {
            continue;
        };

        if action_waiting_for_id == prayer_id {
            let result = found_item_id_maybe.into();
            agent.cpu.stack.push(result);
            agent.state = AgentState::Running;
        }
    }

    for FindNearestResult {
        agent_id,
        prayer_id,
        found_item_id_maybe,
    } in find_nearest_results.read()
    {
        let Ok((_, mut agent)) = query.get_mut(*agent_id) else {
            continue;
        };
        let AgentState::WaitForAction(action_waiting_for_id) = &agent.state else {
            continue;
        };

        if action_waiting_for_id == prayer_id {
            let result: StackItem = match found_item_id_maybe {
                Some((x, _)) => Some(x),
                None => None,
            }
            .into();
            agent.cpu.stack.push(result);
            agent.state = AgentState::Running;
        }
    }

    for GetEnergyResult {
        agent_id,
        prayer_id,
        energy_maybe,
    } in get_energy_result.read()
    {
        let Ok((_, mut agent)) = query.get_mut(*agent_id) else {
            continue;
        };
        let AgentState::WaitForAction(action_waiting_for_id) = &agent.state else {
            continue;
        };

        if action_waiting_for_id == prayer_id {
            let result: StackItem = energy_maybe.into();
            agent.cpu.stack.push(result);
            agent.state = AgentState::Running;
        }
    }

    for GetLocationResult {
        agent_id,
        prayer_id,
        location_maybe,
    } in get_location_result.read()
    {
        let Ok((_, mut agent)) = query.get_mut(*agent_id) else {
            continue;
        };
        let AgentState::WaitForAction(action_waiting_for_id) = &agent.state else {
            continue;
        };

        if action_waiting_for_id == prayer_id {
            let result: StackItem = match location_maybe {
                Some(Location::Inventory(containter_id)) => Some(StackItem::EntityId(*containter_id)),
                Some(Location::World { x, y }) => Some(StackItem::Coord { x: x.into(), y: y.into() }),
                None => None,
            }.into();
            agent.cpu.stack.push(result);
            agent.state = AgentState::Running;
        }
    }

    for (agent_id, mut agent) in &mut query {
        let Agent {
            cpu,
            blackboard,
            bt,
            state,
        } = agent.as_mut();
        match state {
            AgentState::Running => match cpu.step(bt, blackboard) {
                Ok(ok) => {
                    handle_prayer(
                        &mut made_world_query,
                        agent_id,
                        ok,
                        cpu,
                        &mut drop_request,
                        &mut goto_request,
                        &mut find_in_inventory_request,
                        &mut find_nearest_request,
                        &mut get_energy_request,
                        &mut get_hp_request,
                        &mut get_location_request,

                        &mut use_on_request,
                        state,
                    );
                }
                Err(_) => todo!(),
            },
            AgentState::WaitForAction(_prayer_id) => continue,
        }
    }

    if made_world_query {
        let new_loop_count = if let GameState::SimulationPausedForAI(loop_count) = state.get() {
            loop_count + 1
        } else {
            0
        };
        next_state.set(GameState::SimulationPausedForAI(new_loop_count));
    } else {
        next_state.set(GameState::RunningSimulation);
    };
}
