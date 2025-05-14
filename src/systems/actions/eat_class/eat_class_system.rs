use std::{
    collections::BTreeSet,
    hash::{DefaultHasher, Hash, Hasher},
};

use bevy::prelude::*;

use qol::placeholder;

use ethnolib::sandbox::{Item, world::Type};

use crate::systems::{
    actions::{ActionResult, EatClassRequest, Result},
    cache::CacheInventory,
    change_request::{ChangeRequest, Changes},
};

pub fn eat_class_system(
    query: Query<(Option<&CacheInventory>, Option<&Type>)>,
    mut requests: EventReader<EatClassRequest>,
    //mut posible_actions_requests: EventReader<PosibleActionsRequest>,
    //mut posible_actions_responce: EventWriter<PosibleActionsResponce>,
    mut commands: Commands,
) {
    let salt = 0;

    'outer: for EatClassRequest {
        agent_id,
        prayer_id,
        item_class,
    } in requests.read()
    {
        // first we get the cache of the agent's inventory
        if let Ok((Some(CacheInventory { inventory }), _)) = query.get(*agent_id) {
            'inner: for x in inventory.iter() {
                let Ok((_, Some(Type(item_type)))) = query.get(*x) else {
                    continue 'inner;
                };
                if item_type == item_class {
                    let target_id = *x;
                    if eatable(item_type) {
                        let mut s = DefaultHasher::new();
                        salt.hash(&mut s);
                        "EatClass".hash(&mut s);
                        agent_id.hash(&mut s);
                        target_id.hash(&mut s);
                        let hash = s.finish();

                        commands.send_event(ChangeRequest {
                            prayer_id: *prayer_id,
                            hash,
                            contentious_entities: BTreeSet::from([target_id]),
                            changes: placeholder!(vec![Changes::Despawn(target_id)]),
                        });
                        continue 'outer;
                    }
                }
                // couldn't fins sutable
                let result = ActionResult {
                    agent_id: *agent_id,
                    prayer_id: *prayer_id,
                    result: Result::Failure,
                };
                commands.send_event(result);
            }
        } else {
            // agent_id did not have an inventory we should check for objects withing reach but I'm not implemting that right now.
            let result = ActionResult {
                agent_id: *agent_id,
                prayer_id: *prayer_id,
                result: Result::Failure,
            };
            commands.send_event(result);
        }
    }
}

fn eatable(item_type: &Item) -> bool {
    item_type == &Item::Veggie
}
