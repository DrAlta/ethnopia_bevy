use bevy::prelude::*;
use ethnolib::sandbox::world::Type;

use crate::systems::CacheInventory;

use super::{GetIsInventoryGERequest, GetIsInventoryGEResult};

pub fn get_is_inventory_ge_system(
    query: Query<(Option<&CacheInventory>, Option<&Type>)>,
    mut requests: EventReader<GetIsInventoryGERequest>,
    mut result: EventWriter<GetIsInventoryGEResult>,

) {
    for GetIsInventoryGERequest { agent_id, prayer_id, subject_id, item_class, amount } in requests.read() {
        let Ok((Some(inventory), _)) = query.get(*subject_id) else {
            result.send(GetIsInventoryGEResult { agent_id: *agent_id, prayer_id: *prayer_id, ge_maybe: false });
            continue
        };
        let mut count = 0;
        for x in &inventory.inventory {
            let Ok((_, Some(Type(item_type))))= query.get(*x) else {
                continue
            };
            if item_type == item_class {
                count += 1;
            }
        }
        result.send(GetIsInventoryGEResult {
            agent_id: *agent_id,
            prayer_id: *prayer_id,
            ge_maybe: &count >= amount,
        });

    }

}