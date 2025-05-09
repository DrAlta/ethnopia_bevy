use crate::{
    GameState,
    systems::{
        actions::{DropRequest, GotoRequest, UseOnRequest},
        agent::{Agent, AgentState},
        query::{
            FindInInventoryRequest, FindNearestRequest, GetEnergyRequest, GetEntitiesRequest,
            GetHpRequest, GetIsInventoryGERequest, GetLocationRequest,
        },
    },
};
use bevy::prelude::*;

use super::handle_prayer;

pub fn agent_system(
    mut query: Query<(Entity, &mut Agent)>,

    mut drop_request: EventWriter<DropRequest>,
    mut goto_request: EventWriter<GotoRequest>,
    mut find_in_inventory_request: EventWriter<FindInInventoryRequest>,
    mut find_nearest_request: EventWriter<FindNearestRequest>,
    mut get_energy_request: EventWriter<GetEnergyRequest>,
    mut get_entities_request: EventWriter<GetEntitiesRequest>,
    mut get_hp_request: EventWriter<GetHpRequest>,
    mut get_is_inventory_ge_request: EventWriter<GetIsInventoryGERequest>,
    mut get_location_request: EventWriter<GetLocationRequest>,
    mut use_on_request: EventWriter<UseOnRequest>,

    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    //mut commands: Commands,
) {
    let mut made_world_query = false;

    for (agent_id, mut agent) in &mut query {
        let Agent {
            cpu,
            blackboard,
            bt,
            state,
            main,
        } = agent.as_mut();
        match state {
            AgentState::Running => match cpu.step(bt, blackboard) {
                Ok(ok) => {
                    handle_prayer(
                        &mut made_world_query,
                        agent_id,
                        main,
                        ok,
                        cpu,
                        &mut drop_request,
                        &mut goto_request,
                        &mut find_in_inventory_request,
                        &mut find_nearest_request,
                        &mut get_energy_request,
                        &mut get_entities_request,
                        &mut get_hp_request,
                        &mut get_is_inventory_ge_request,
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
