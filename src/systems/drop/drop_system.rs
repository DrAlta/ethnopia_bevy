use bevy::prelude::*;
use ethnolib::sandbox::{actions::ActionResult, Location};

use super::DropRequest;

pub fn drop_system(
    mut query: Query<&mut Location>,
    mut drop_requests: EventReader<DropRequest>,
    mut result: EventWriter<ActionResult>,
) {
    for DropRequest{ agent_id, prayer_id, object_id } in drop_requests.read(){
        let (
            Ok(&Location::World { x, y }), 
            Ok(Location::Inventory(objects_container_id))
        ) = (
            query.get(*agent_id), 
            query.get(*object_id)
        ) else{
            result.send(ActionResult{ 
                agent_id: *agent_id,
                prayer_id: *prayer_id,
                result: ethnolib::sandbox::actions::Result::Failure
            });
            continue
        };
        if objects_container_id != agent_id {
            result.send(ActionResult{ 
                agent_id: *agent_id,
                prayer_id: *prayer_id,
                result: ethnolib::sandbox::actions::Result::Failure
            });
            continue
        }
        let Ok(mut object_loc) = query.get_mut(*object_id) else{
            result.send(ActionResult{ 
                agent_id: *agent_id,
                prayer_id: *prayer_id,
                result: ethnolib::sandbox::actions::Result::Failure
            });
            continue
        };
        *object_loc = Location::World { x, y };
        result.send(ActionResult{ 
            agent_id: *agent_id,
            prayer_id: *prayer_id,
            result: ethnolib::sandbox::actions::Result::Success
        });
    }
}