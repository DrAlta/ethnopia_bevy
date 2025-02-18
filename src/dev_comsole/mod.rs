use bevy::prelude::*;
use ethnolib::sandbox::Collision;

pub fn collision_report_system(
    mut events: EventReader<Collision>,
) {

    for Collision { agent_id, collider_id } in events.read() {
        println!("{agent_id} collised with {collider_id}")
    }
}