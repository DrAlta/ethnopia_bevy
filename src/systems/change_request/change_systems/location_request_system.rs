use bevy::prelude::*;
use ethnolib::sandbox::Location;
use qol::logy;

use crate::systems::change_request::ChangeLocation;

pub fn location_request_system(
    mut query: Query<&mut Location>,
    mut requests: EventReader<ChangeLocation>,
    mut commands: Commands,
) {
    for ChangeLocation { entity_id, location } in requests.read() {
        let Ok(mut entity_location) = query.get_mut(*entity_id) else {
            logy!("trace-req-location", "adding location on entity {entity_id:?}");
            commands.entity(*entity_id).insert(location.clone());
            continue;
        };
        logy!("trace-req-location", "modifing location on entity {entity_id:?}");
        *entity_location = location.clone();
    }
}
