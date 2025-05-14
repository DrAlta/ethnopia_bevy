use bevy::prelude::*;
use ethnolib::sandbox::{ai::StackItem, world::Type};

use super::{RemoveEntitiesOfClassRequest, RemoveEntitiesOfClassResult};

pub fn remove_entities_of_class_system(
    query: Query<&Type>,
    mut requests: EventReader<RemoveEntitiesOfClassRequest>,
    mut results: EventWriter<RemoveEntitiesOfClassResult>,
) {
    for RemoveEntitiesOfClassRequest {
        agent_id,
        prayer_id,
        item_class,
        table,
    } in requests.read()
    {
        let new_table = ethnolib::sandbox::ai::TableInterior {
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
                    if tyep == item_class {
                        None
                    } else {
                        Some((k.clone(), v.clone()))
                    }
                })
                .collect(),
        };
        let result = RemoveEntitiesOfClassResult {
            agent_id: *agent_id,
            prayer_id: *prayer_id,
            table: new_table,
        };
        results.send(result);
    }
}
