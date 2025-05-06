use bevy::prelude::*;
use ethnolib::sandbox::Location;

use crate::systems::query::{GetLocationRequest, GetLocationResult};

pub fn get_location_system(
    query: Query<&Location>,
    mut requests: EventReader<GetLocationRequest>,
    mut result: EventWriter<GetLocationResult>,
) {
    for GetLocationRequest {
        agent_id,
        prayer_id,
        subject_id,
    } in requests.read()
    {
        let location_maybe = if let Ok(location) = query.get(*subject_id) {
            Some(location.clone())
        } else {
            None
        };
        let event = GetLocationResult {
            agent_id: *agent_id,
            prayer_id: *prayer_id,
            location_maybe,
        };
        result.send(event);
    }
}
