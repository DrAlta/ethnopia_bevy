use crate::systems::{
    actions::{self, ActionResult},
    agent::{Agent, AgentState},
    query::{
        FindInInventoryResult, FindNearestResult, GetEnergyResult, GetIsInventoryGEResult,
        GetLocationResult,
    },
};
use bevy::prelude::*;
use ethnolib::sandbox::{Location, ai::StackItem};

pub fn receive_prayers_system(
    mut query: Query<(Entity, &mut Agent)>,
    mut action_results: EventReader<ActionResult>,
    mut find_in_inventory_results: EventReader<FindInInventoryResult>,
    mut find_nearest_results: EventReader<FindNearestResult>,
    mut get_energy_result: EventReader<GetEnergyResult>,
    mut get_is_inventory_ge_result: EventReader<GetIsInventoryGEResult>,
    mut get_location_result: EventReader<GetLocationResult>,
) {
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

    for GetIsInventoryGEResult {
        agent_id,
        prayer_id,
        ge_maybe,
    } in get_is_inventory_ge_result.read()
    {
        let Ok((_, mut agent)) = query.get_mut(*agent_id) else {
            continue;
        };
        let AgentState::WaitForAction(action_waiting_for_id) = &agent.state else {
            continue;
        };

        if action_waiting_for_id == prayer_id {
            let result: StackItem = ge_maybe.into();
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
                Some(Location::Inventory(containter_id)) => {
                    Some(StackItem::EntityId(*containter_id))
                }
                Some(Location::World { x, y }) => Some(StackItem::Coord {
                    x: x.into(),
                    y: y.into(),
                }),
                None => None,
            }
            .into();
            agent.cpu.stack.push(result);
            agent.state = AgentState::Running;
        }
    }
}
