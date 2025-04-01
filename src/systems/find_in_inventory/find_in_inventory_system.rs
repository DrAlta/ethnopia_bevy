use bevy::prelude::*;
use ethnolib::sandbox::world::Type;

use crate::systems::{
    cache_inventory::CacheInventory,
    find_in_inventory::{FindInInventoryRequest, FindInInventoryResult},
};

pub fn find_in_inventory_system(
    query: Query<(Option<&CacheInventory>, Option<&Type>)>,
    mut requests: EventReader<FindInInventoryRequest>,
    mut results: EventWriter<FindInInventoryResult>,
) {
    for FindInInventoryRequest {
        agent_id,
        prayer_id,
        item_class,
    } in requests.read()
    {
        // first we get the cache of the agent's inventory
        if let Ok((Some(CacheInventory { inventory }), _)) = query.get(*agent_id) {
            // find an item of the right type
            let found_item_id_maybe = inventory
                .iter()
                .find(|x| {
                    let Ok((_, Some(Type(item_type)))) = query.get(**x) else {
                        return false;
                    };
                    item_type == item_class
                })
                .cloned();

            // send the result of what we found
            results.send(FindInInventoryResult {
                agent_id: *agent_id,
                prayer_id: *prayer_id,
                found_item_id_maybe,
            });
        } else {
            // agent_id did not have an inventory so re just responed that no item was found
            results.send(FindInInventoryResult {
                agent_id: *agent_id,
                prayer_id: *prayer_id,
                found_item_id_maybe: None,
            });
        }
    }
}
