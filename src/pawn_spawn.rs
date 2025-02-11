use bevy::{math::vec2, color::palettes::css::{BLUE, BROWN, GRAY, GREEN, ORANGE, PINK, RED, YELLOW}, prelude::*};
use ethnolib::sandbox::{world::{Size, Type}, Item, Location};

#[derive(Debug, Clone, PartialEq, Component)]
pub struct Pawn(Entity);

pub fn pawn_spawn(
    mut pawn_query: Query<(Entity, Option<&mut Transform>, Option<&mut Sprite>)>,
    item_query: Query<(Entity, Option<&Type>, &Location, Option<&Size>, Option<&Pawn>)>,
    mut commands: Commands
) {
    let mut done = Vec::new();
    for (entity_id, tyep_maybe, location, size_maybe, pawn_maybe) in item_query.iter() {
        // if the entity is in the world make sure it has a paen
        if let &Location::World { x, y } = location {
            let (width, height) = if let Some(&Size{ width, height }) = size_maybe {
                (width as f32, height as f32)
            } else {
                (crate::BRICK_SIZE, crate::BRICK_SIZE)
            };
            let color = Color::Srgba(if let Some(Type(tyep)) = tyep_maybe {
                const DARKGRAY: Srgba = Srgba::rgb(0.31, 0.31, 0.31);
                const DARKBROWN: Srgba = Srgba::rgb(0.3, 0.25, 0.18);
                match tyep {
                    Item::Agent => BLUE,
                    Item::Axe => GRAY,
                    Item::Food => YELLOW,
                    Item::Stone => DARKGRAY,
                    Item::Stick => BROWN,
                    Item::Wood => DARKBROWN,
                    Item::House => RED,
                    Item::Tree => GREEN,
                    Item::Veggie => ORANGE,
                }
            } else {
                PINK
            });
            let translate = Vec3{x, y, z: 0.0};
            // see if it already has pawn
            if let Some(&Pawn(pawn_id)) = pawn_maybe {
                // it has a paen so try to get it
                if let Ok((_, transform_maybe, sprite_maybe)) = pawn_query.get_mut(pawn_id) {
                    done.push(pawn_id);
                    // the pawn had a transgorm os set the translaion
                    if let Some(mut transform) = transform_maybe {
                        transform.translation = translate;
                    } else {
                    // the pawn didn't have a translation so add one
                        commands.entity(pawn_id).insert(Transform {
                            translation: vec2(x, y).extend(0.0),
                            scale: Vec3::new(width, height, 1.0),
                            ..default()
                        });
                    }
                    // the pawn has a sprite so set it's color
                    if let Some(mut sprite)= sprite_maybe {
                        sprite.color = color;
                    } else {
                    // the pawn didn't have a sprite so create one
                        commands.entity(pawn_id).insert(Sprite {
                            color,
                            ..default()
                        });
                    }
                    //sprite.color = color;
                }
            } else {
                // just is case the Pawn was set to an entity that didn't exist
                if pawn_maybe.is_some() {
                    commands.entity(entity_id).remove::<Pawn>();
                }
                // the entity didn't have a pawn so create one
                let transform = Transform {
                    translation: vec2(x, y).extend(0.0),
                    scale: Vec3::new(width, height, 1.0),
                    ..default()
                };
                let sprite = Sprite {
                    color,
                    ..default()
                };
                let pawn_id = commands.spawn((sprite, transform)).id();
                commands.entity(entity_id).insert(Pawn(pawn_id));
            }

        }

    }
}