use std::collections::HashMap;

use bevy::prelude::*;
use ethnolib::sandbox::Location;
use qol::PushOrInsert;

use crate::systems::cache_inventory::CacheInventory;

pub fn cache_inventory_system(
    mut query: Query<(Entity, Option<&Location>, Option<&mut CacheInventory>)>,
    mut commands: Commands,
) {
    let mut todo = HashMap::new();
    for (object_id, location_maybe, _) in &query {
        match location_maybe {
            Some(Location::Inventory(container_id)) => {
                todo.push_or_insert(*container_id, object_id);
            }
            _ => (),
        }
    }
    for (container_id, _, cache_inventory_maybe) in &mut query {
        let Some(mut cache_inventory) = cache_inventory_maybe else {
            continue;
        };
        let mut inventory = todo.remove(&container_id).unwrap_or(Vec::new());
        inventory.shrink_to_fit();
        cache_inventory.inventory = inventory;
    }

    // todo should now only contain the inventories of containers that didn't have a CacheInventory compone so wE'll add one to them
    for (container_id, mut inventory) in todo {
        inventory.shrink_to_fit();
        commands
            .entity(container_id)
            .insert(CacheInventory { inventory });
    }
}
