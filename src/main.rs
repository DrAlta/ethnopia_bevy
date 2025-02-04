//! AI sends an event to the world_polling system and sets the agents AI to counter of how many ticks it's waited to 0
//! world_polling system recaise the event from the AI syste mt pull the world and does the pulls then send the result back to the AI sysye m in a event
//! The AI first process the events and for pull_resulr events and continues proccess the AI [HA note:pull instustions finish processing on the same tick as the next instruction is run]
//! 
//! the AI system process the agents and if a agent is is waitng for world_pull result then it's counter is incremented then a check ot see if it's been to long and the instruction should fail.
//!  if the agent isn't waiting it ticks the agent's AI
use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, hello_world)
        .run();
}
fn hello_world() {
    println!("hello world!");
}
#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);
fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}
