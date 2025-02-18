//! AI sends an event to the world_polling system and sets the agents AI to counter of how many ticks it's waited to 0
//! world_polling system recaise the event from the AI syste mt pull the world and does the pulls then send the result back to the AI sysye m in a event
//! The AI first process the events and for pull_resulr events and continues proccess the AI [HA note:pull instustions finish processing on the same tick as the next instruction is run]
//!
//! the AI system process the agents and if a agent is is waitng for world_pull result then it's counter is incremented then a check ot see if it's been to long and the instruction should fail.
//!  if the agent isn't waiting it ticks the agent's AI
use bevy::prelude::*;

use dev_comsole::collision_report_system;
use ethnolib::{
    sandbox::{
        actions::{use_object_system, PosibleActionsRequest, PosibleActionsResponce, UseRequest}, change_request::{
            change_request_system, ChangeConflict, ChangeDespawn, ChangeEnergy, ChangeHp, ChangeRequest, ChangeSpawnLocationType
        }, process_movement, world::{Energy, Hp, Item, Movement, Size, Type}, Collision, TravelCompleted, Location
    }, Number, Vec2
};

mod dev_comsole;
mod pawn_spawn;
use pawn_spawn::pawn_spawn;
mod salt;
pub use salt::{Salt,  salt_system};

const CELL_SIZE: Number = 30.0;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Salt(0))
        .add_systems(Startup, setup)
        .add_event::<UseRequest>()
        .add_event::<PosibleActionsRequest>()
        .add_event::<PosibleActionsResponce>()
        .add_event::<TravelCompleted>()
        .add_event::<Collision>()
        .add_event::<ChangeRequest>()
        .add_event::<ChangeConflict>()
        .add_event::<ChangeDespawn>()
        .add_event::<ChangeEnergy>()
        .add_event::<ChangeHp>()
        .add_event::<ChangeSpawnLocationType>()
        .add_systems(
            Update,
            (
                salt_system,
                process_movement,
                collision_report_system,
                (use_object_system, change_request_system).chain(),
                pawn_spawn,
            )
                .chain(),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    let agent_id = commands
        .spawn((
            Type(Item::Agent),
            Location::World { x: 0.0, y: 0.0 },
            Size {
                width: CELL_SIZE as i32,
                height: CELL_SIZE as i32,
            },
            Hp(10),
            Energy(10),
            Movement{ target: Vec2{x: 200.0, y: 5.0}, speed: 5.0 }
        ))
        .id();
    commands.spawn((Type(Item::Axe), Location::Inventory(agent_id), Size {
        width: CELL_SIZE as i32,
        height: CELL_SIZE as i32,
    }));

    commands.spawn((
        Type(Item::Tree),
        Location::World {
            x: CELL_SIZE * 3.0,
            y: 0.0,
        },
        Size {
            width: CELL_SIZE as i32,
            height: CELL_SIZE as i32,
        },
    ));
    /*
        let mut world = World::from((
            HashMap::from([
                (0, ),
                (1, Location::Inventory(0)),
                (2, Location::World { x: 0.0, y: 19.0 }),
            ]),

            HashMap::from([
                (0, (GRID_SIZE, GRID_SIZE)),
                (2, (GRID_SIZE, GRID_SIZE)),
            ]),
            HashMap::from([(0, Item::Agent), (1, Item::Axe), (2, Item::Tree)]),
    */
}
