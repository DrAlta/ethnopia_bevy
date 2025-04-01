use bevy::prelude::*;
use ethnolib::sandbox::Collision;
use qol::logy;

pub fn collision_report_system(
    mut events: EventReader<Collision>,
) {

    for Collision { agent_id, collider_id } in events.read() {
        logy!("log", "{agent_id} collised with {collider_id}")
    }
}