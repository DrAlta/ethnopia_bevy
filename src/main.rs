//! AI sends an event to the world_polling system and sets the agents AI to counter of how many ticks it's waited to 0
//! world_polling system recaise the event from the AI syste mt pull the world and does the pulls then send the result back to the AI sysye m in a event
//! The AI first process the events and for pull_resulr events and continues proccess the AI [HA note:pull instustions finish processing on the same tick as the next instruction is run]
//! 
//! the AI system process the agents and if a agent is is waitng for world_pull result then it's counter is incremented then a check ot see if it's been to long and the instruction should fail.
//!  if the agent isn't waiting it ticks the agent's AI
use bevy::{
    math::vec2,
    prelude::*,
};


mod ai_system;
pub use ai_system::ai_system;
mod entities_in_area_system;
pub use entities_in_area_system::entities_in_area_system;

const BRICK_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);
const BRICK_SIZE: Vec2 = Vec2::new(100., 30.);


type Coord = (i32,i32);
#[derive(Event, Debug)]
pub struct EntitiesInAreaReponse {
    pub agent_id: Entity,
    pub  entitie_ids: Vec<Entity>,
}
#[derive(Event, Debug)]
pub struct EntitiesInAreaRequest {
    pub agent_id: Entity,
    pub pos: Coord,
    pub size: Coord,
}




fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_event::<EntitiesInAreaReponse>()
    .add_event::<EntitiesInAreaRequest>()
    .add_systems(Startup, setup)
    .add_systems(Update, ai_system)
    .add_systems(Update, entities_in_area_system)
    .run();
}

#[derive(Component)]
pub struct Size(Coord);

#[derive(Component)]
pub struct Person;

#[derive(Component)]
pub struct Name(String);

#[derive(Component)]
pub struct AI(u8, u8);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    add_people(commands);
}

fn add_people(mut commands: Commands) {

    commands.spawn((
        Person, 
        Name("Elaina Proctor".to_string()),
        AI(0, 14),
        Sprite {
            color: BRICK_COLOR,
            ..default()
        },
        Transform {
            translation: vec2(0.0, 0.0).extend(0.0),
            scale: Vec3::new(BRICK_SIZE.x, BRICK_SIZE.y, 1.0),
            ..default()
        },
        Size((BRICK_SIZE.x as i32, BRICK_SIZE.y as i32)),
    ));
    commands.spawn((
        Person, 
        Name("Renzo Hume".to_string()),
        AI(0, 1),
        Sprite {
            color: BRICK_COLOR,
            ..default()
        },
        Transform {
            translation: vec2(BRICK_SIZE.x + 5.0, 0.0).extend(0.0),
            scale: Vec3::new(BRICK_SIZE.x, BRICK_SIZE.y, 1.0),
            ..default()
        },
        Size((BRICK_SIZE.x as i32, BRICK_SIZE.y as i32)),
    ));
    commands.spawn((
        Person, 
        Name("Zayna Nieves".to_string()),
        AI(10, 10),
        Sprite {
            color: BRICK_COLOR,
            ..default()
        },
        Transform {
            translation: vec2(BRICK_SIZE.x + 5.0, BRICK_SIZE.y + 5.0).extend(0.0),
            scale: Vec3::new(BRICK_SIZE.x, BRICK_SIZE.y, 1.0),
            ..default()
        },
        Size((BRICK_SIZE.x as i32, BRICK_SIZE.y as i32)),
    ));
}
