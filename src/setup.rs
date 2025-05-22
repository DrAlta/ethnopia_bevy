use bevy::prelude::*;
use ethnolib::{
    Number,
    sandbox::{
        Item, Location,
        ai::{Blackboard, BlackboardValue},
        world::{Energy, Hp, Size, Type},
    },
};

use crate::CELL_SIZE;

pub fn setup(mut commands: Commands) {
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
    crate::systems::Agent::add(
        "hermit".to_owned(),
        ethnolib::sandbox::ai::get_hermit_behavior_task(),
        agent_id,
        &mut commands,
    );

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
