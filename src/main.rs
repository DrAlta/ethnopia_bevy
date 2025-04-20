//! AI sends an event to the world_polling system and sets the agents AI to counter of how many ticks it's waited to 0
//! world_polling system recaise the event from the AI syste mt pull the world and does the pulls then send the result back to the AI sysye m in a event
//! The AI first process the events and for pull_resulr events and continues proccess the AI [HA note:pull instustions finish processing on the same tick as the next instruction is run]
//!
//! the AI system process the agents and if a agent is is waitng for world_pull result then it's counter is incremented then a check ot see if it's been to long and the instruction should fail.
//!  if the agent isn't waiting it ticks the agent's AI
use bevy::{
    color::palettes::css::{BLUE, BROWN, GRAY, GREEN, ORANGE, RED, STEEL_BLUE, TAN, YELLOW},
    prelude::*,
};
use ethnolib::{
    Number,
    sandbox::{
        Item, Location,
        movement::Collision,
        world::{Energy, Hp, Size, Type},
    },
};

use crate::systems::{
    actions::{
        ActionResult, GotoRequest, UseOnRequest, UseRequest, goto_system, use_object_system,
    },
    change_request::{
        ChangeConflict, ChangeDespawn, ChangeEnergy, ChangeHp, ChangeRequest,
        ChangeSpawnLocationType, change_request_system,
    },
    movement::{TravelCompleted, process_movement},
    posible_actions::{PosibleActionsRequest, PosibleActionsResponce},
};

mod dev_console;
use dev_console::collision_report_system;
pub mod systems;
use systems::{
    Salt, agent_system, cache_inventory_system,
    query::{FindInInventoryRequest, FindInInventoryResult, find_in_inventory_system},
    salt_system,
};
mod pawn_spawn;
use pawn_spawn::pawn_spawn;
//mod picking;
//use picking::{cursor_hovering_system, CursorOnPawn, Picky};
mod ui;
pub use ui::menu;
use ui::{hover_out_system, picking_system};

const CELL_SIZE: Number = Number::new(30, 1);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(systems::BVH(None))
        .insert_resource(Salt(0))
        .insert_resource(ui::UIState {
            selected_entity: None,
            mode: ui::Mode::Pan,
            actions: Vec::new(),
        })
        .add_systems(Startup, setup)
        // movement
        .add_event::<TravelCompleted>()
        .add_event::<Collision>()
        // actions
        .add_event::<GotoRequest>()
        .add_event::<UseRequest>()
        .add_event::<UseOnRequest>()
        // action results
        .add_event::<ActionResult>()
        // posible actions
        .add_event::<PosibleActionsRequest>()
        .add_event::<PosibleActionsResponce>()
        // changes
        .add_event::<ChangeRequest>()
        .add_event::<ChangeConflict>()
        .add_event::<ChangeDespawn>()
        .add_event::<ChangeEnergy>()
        .add_event::<ChangeHp>()
        .add_event::<ChangeSpawnLocationType>()
        // prayers
        .add_event::<FindInInventoryRequest>()
        .add_event::<FindInInventoryResult>()
        // ui
        .add_event::<ui::UISelect>()
        .add_systems(
            Update,
            (
                salt_system,
                // do the world simulation
                // movement systems
                process_movement,
                collision_report_system,
                (
                    // action systems
                    use_object_system,
                ),
                // resolve the changes that actions wanted to have on the world
                change_request_system,
                // world sim finished
                (
                    // build caches
                    cache_inventory_system,
                    systems::bvh_system,
                ),
                (
                    // answer prayers
                    goto_system,
                    find_in_inventory_system,
                ),
                // run AI
                agent_system,
                // UI
                pawn_spawn,
                picking_system,
                hover_out_system,
                ui::ui_system,
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
            Location::World {
                x: Number::ZERO,
                y: Number::ZERO,
            },
            Size {
                width: Into::<i32>::into(CELL_SIZE),
                height: Into::<i32>::into(CELL_SIZE),
            },
            Hp(10),
            Energy(10),
            //    Movement{ target: Vec2{x: 200.0, y: 5.0}, speed: 5.0 }
        ))
        .id();
    commands.spawn((Type(Item::Axe), Location::Inventory(agent_id), Size {
        width: Into::<i32>::into(CELL_SIZE),
        height: Into::<i32>::into(CELL_SIZE),
    }));

    commands.spawn((
        Type(Item::Tree),
        Location::World {
            x: CELL_SIZE * 3.0,
            y: Number::ZERO,
        },
        Size {
            width: Into::<i32>::into(CELL_SIZE),
            height: Into::<i32>::into(CELL_SIZE),
        },
    ));
    commands.spawn((
        Text2d::new("hello world!"),
        Transform::from_xyz(0.0, 30.0, 0.0),
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

pub fn type_to_color(tyep: &Item) -> Color {
    const DARKGRAY: Srgba = Srgba::rgb(0.31, 0.31, 0.31);
    const DARKBROWN: Srgba = Srgba::rgb(0.3, 0.25, 0.18);
    Color::Srgba(match tyep {
        Item::Agent => BLUE,
        Item::Axe => GRAY,
        Item::Food => YELLOW,
        Item::Stone => DARKGRAY,
        Item::Stick => BROWN,
        Item::Wood => DARKBROWN,
        Item::House => RED,
        Item::Tree => GREEN,
        Item::Veggie => ORANGE,
        Item::Knife => STEEL_BLUE,
        Item::Seed => TAN,
    })
}
