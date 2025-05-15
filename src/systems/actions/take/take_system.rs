use std::collections::BTreeSet;
use std::hash::{DefaultHasher, Hash, Hasher};

use bevy::prelude::*;

use ethnolib::{sandbox::{EntityId, Location}, Number};
use qol::placeholder;

use crate::systems::{actions::{ActionResult, Result, TakeRequest}, change_request::{ChangeRequest, Changes}};

pub fn take_system(
    query: Query<&Location>,

    mut requests: EventReader<TakeRequest>,
    mut commands: Commands,
) {
    let salt = 0;
    for TakeRequest { agent_id, prayer_id, object_id } in requests.read() {
        if agent_id == object_id {
            // can't take one's self
            let result = ActionResult {
                agent_id: *agent_id,
                prayer_id: *prayer_id,
                result: Result::Failure,
            };
            commands.send_event(result);
            continue;
        };
        let Ok(agent_location) = query.get(*agent_id) else {
            // couldn't find agent's location
            let result = ActionResult {
                agent_id: *agent_id,
                prayer_id: *prayer_id,
                result: Result::Failure,
            };
            commands.send_event(result);
            continue;
        };
        let Ok(object_location) = query.get(*object_id) else {
            // couldn't find agent's location
            let result = ActionResult {
                agent_id: *agent_id,
                prayer_id: *prayer_id,
                result: Result::Failure,
            };
            commands.send_event(result);
            continue;
        };
        match (agent_location, object_location) {
            (Location::Inventory(al), Location::Inventory(ol)) => {
                if al == ol {
                    foo(*prayer_id, *agent_id, *object_id, salt, &mut commands);
                    continue;
                }
            },
            (Location::Inventory(_), Location::World { .. }) => (),
            (Location::World { .. }, Location::Inventory(_)) => (),
            (Location::World { x:ax, y: ay }, Location::World { x: ox, y:oy }) => {
                let dx= ax - ox;
                let dy = ay - oy;
                let dist_sq = (dx * dx) + (dy * dy);
                if dist_sq <= Into::<Number>::into(100) {
                    foo(*prayer_id, *agent_id, *object_id, salt, &mut commands);
                    continue;
                }
            },
        };
        let result = ActionResult {
            agent_id: *agent_id,
            prayer_id: *prayer_id,
            result: Result::Failure,
        };
        commands.send_event(result);
    }
}

fn foo(prayer_id: u64, agent_id: EntityId, object_id: EntityId, salt: u64, commands: &mut Commands) {
    let mut s = DefaultHasher::new();
    salt.hash(&mut s);
    "Take".hash(&mut s);
    agent_id.hash(&mut s);
    object_id.hash(&mut s);
    let hash = s.finish();

    commands.send_event(ChangeRequest {
        prayer_id,
        hash,
        contentious_entities: BTreeSet::from([object_id]),
        changes: placeholder!(vec![
            Changes::Location {
                entity_id: object_id,
                location: Location::Inventory(agent_id)
            }
        ]),
    });
}