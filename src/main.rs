//! AI sends an event to the world_polling system and sets the agents AI to counter of how many ticks it's waited to 0
//! world_polling system recaise the event from the AI syste mt pull the world and does the pulls then send the result back to the AI sysye m in a event
//! The AI first process the events and for pull_resulr events and continues proccess the AI [HA note:pull instustions finish processing on the same tick as the next instruction is run]
//!
//! the AI system process the agents and if a agent is is waitng for world_pull result then it's counter is incremented then a check ot see if it's been to long and the instruction should fail.
//!  if the agent isn't waiting it ticks the agent's AI
use bevy::prelude::*;

mod entities_in_area_system;
pub use entities_in_area_system::entities_in_area_system;
use ethnolib::{sandbox::{
    actions::{use_object_system, PosibleActionsRequest, PosibleActionsResponce, UseRequest}, change_request::change_request_system, world::{Energy, Hp, Item, Size, Type}, Location
}, Number};

mod pawn_spawn;
use pawn_spawn::pawn_spawn;

const BRICK_SIZE: Number = 30.0 ;

type Coord = (i32, i32);

#[derive(Event, Debug)]
pub struct EntitiesInAreaReponse {
    pub agent_id: Entity,
    pub entitie_ids: Vec<Entity>,
}

#[derive(Event, Debug)]
pub struct EntitiesInAreaRequest {
    pub agent_id: Entity,
    pub pos: Coord,
    pub size: Coord,
}

#[derive(Debug, Resource)]
pub struct Salt(u64);

pub fn salt_system(mut salt: ResMut<Salt>) {
    salt.0 += 1;
    println!("----salt {}", salt.0);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Salt(0))
        .add_systems(Startup, setup)
        .add_event::<UseRequest>()
        .add_event::<PosibleActionsRequest>()
        .add_event::<PosibleActionsResponce>()
        .add_systems(Update, (salt_system, (use_object_system, change_request_system).chain(), pawn_spawn).chain())
        .run();
}


fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    let agent_id = commands.spawn((
        Type(Item::Agent),
        Location::World { x: 0.0, y: 0.0 },
        Size{width:BRICK_SIZE as i32, height:BRICK_SIZE as i32},
        Hp(10),
        Energy(10),
    )).id();
    commands.spawn((
        Type(Item::Axe),
        Location::Inventory(agent_id),
        Size{width:BRICK_SIZE as i32, height:BRICK_SIZE as i32},
    ));

    commands.spawn((
        Type(Item::Tree),
        Location::World { x: BRICK_SIZE * 3.0, y: 0.0 },
        Size{width:BRICK_SIZE as i32, height:BRICK_SIZE as i32},
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