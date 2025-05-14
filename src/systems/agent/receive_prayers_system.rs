use std::sync::Arc;

use crate::systems::{
    actions::{self, ActionResult},
    agent::{Agent, AgentState},
    query::{
        FindInInventoryResult, FindNearestResult, GetEnergyResult, GetEntitiesResult,
        GetIsInventoryGEResult, GetLocationResult, RemoveEntitiesOfClassResult,
        RetainEntitiesOfClassResult,
    },
};
use bevy::prelude::*;
use ethnolib::sandbox::{Location, ai::StackItem};

pub fn receive_prayers_system(
    mut query: Query<&mut Agent>,
    mut action_results: EventReader<ActionResult>,
    mut find_in_inventory_results: EventReader<FindInInventoryResult>,
    mut find_nearest_results: EventReader<FindNearestResult>,
    mut get_energy_results: EventReader<GetEnergyResult>,
    mut get_entities_results: EventReader<GetEntitiesResult>,
    mut get_is_inventory_ge_results: EventReader<GetIsInventoryGEResult>,
    mut get_location_results: EventReader<GetLocationResult>,
    mut remove_entities_of_class_results: EventReader<RemoveEntitiesOfClassResult>,
    mut retain_entities_of_class_results: EventReader<RetainEntitiesOfClassResult>,
) {
    for ActionResult {
        agent_id,
        prayer_id,
        result,
    } in action_results.read()
    {
        let Ok(mut agent) = query.get_mut(*agent_id) else {
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
        let Ok(mut agent) = query.get_mut(*agent_id) else {
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
        let Ok(mut agent) = query.get_mut(*agent_id) else {
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
    } in get_energy_results.read()
    {
        let Ok(mut agent) = query.get_mut(*agent_id) else {
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

    for GetEntitiesResult {
        agent_id,
        prayer_id,
        entities,
    } in get_entities_results.read()
    {
        let Ok(mut agent) = query.get_mut(*agent_id) else {
            continue;
        };
        let AgentState::WaitForAction(action_waiting_for_id) = &agent.state else {
            continue;
        };

        if action_waiting_for_id == prayer_id {
            let iter = entities.into_iter();
            let result = StackItem::from_iter(iter);
            agent.cpu.stack.push(result);
            agent.state = AgentState::Running;
        }
    }

    for GetIsInventoryGEResult {
        agent_id,
        prayer_id,
        ge_maybe,
    } in get_is_inventory_ge_results.read()
    {
        let Ok(mut agent) = query.get_mut(*agent_id) else {
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
    } in get_location_results.read()
    {
        let Ok(mut agent) = query.get_mut(*agent_id) else {
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
    for RemoveEntitiesOfClassResult {
        agent_id,
        prayer_id,
        table,
    } in remove_entities_of_class_results.read()
    {
        let Ok(mut agent) = query.get_mut(*agent_id) else {
            continue;
        };
        let AgentState::WaitForAction(action_waiting_for_id) = &agent.state else {
            continue;
        };

        if action_waiting_for_id == prayer_id {
            let result: StackItem = StackItem::Table(Arc::new(table.clone()));
            agent.cpu.stack.push(result);
            agent.state = AgentState::Running;
        }
    }
    for RetainEntitiesOfClassResult {
        agent_id,
        prayer_id,
        table,
    } in retain_entities_of_class_results.read()
    {
        let Ok(mut agent) = query.get_mut(*agent_id) else {
            continue;
        };
        let AgentState::WaitForAction(action_waiting_for_id) = &agent.state else {
            continue;
        };

        if action_waiting_for_id == prayer_id {
            let result: StackItem = StackItem::Table(Arc::new(table.clone()));
            agent.cpu.stack.push(result);
            agent.state = AgentState::Running;
        }
    }
}
