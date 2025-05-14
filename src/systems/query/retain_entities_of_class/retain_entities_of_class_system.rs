use bevy::prelude::*;
use ethnolib::sandbox::{
    ai::{StackItem, TableInterior},
    world::Type,
};

use crate::systems::query::{RetainEntitiesOfClassRequest, RetainEntitiesOfClassResult};

pub fn retain_entities_of_class_system(
    query: Query<&Type>,
    mut requests: EventReader<RetainEntitiesOfClassRequest>,
    mut results: EventWriter<RetainEntitiesOfClassResult>,
) {
    for RetainEntitiesOfClassRequest {
        agent_id,
        prayer_id,
        item_class,
        table,
    } in requests.read()
    {
        let new_table = TableInterior {
            map: table
                .map
                .iter()
                .filter_map(|(k, v)| {
                    let StackItem::EntityId(id) = v else {
                        return Some((k.clone(), v.clone()));
                    };
                    let Ok(Type(tyep)) = query.get(*id) else {
                        return Some((k.clone(), v.clone()));
                    };
                    if tyep != item_class {
                        None
                    } else {
                        Some((k.clone(), v.clone()))
                    }
                })
                .collect(),
        };
        let result = RetainEntitiesOfClassResult {
            agent_id: *agent_id,
            prayer_id: *prayer_id,
            table: new_table,
        };
        results.send(result);
    }
}
