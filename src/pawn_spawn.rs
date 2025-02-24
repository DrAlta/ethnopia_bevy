use bevy::{
    color::palettes::css::PINK,
    prelude::*,
};
use ethnolib::{
    Number,
    sandbox::{
        Location,
        world::{Size, Type},
    },
};

#[derive(Debug, Clone, PartialEq, Component)]
pub struct Pawn(pub Entity);

pub fn pawn_spawn(
    mut pawn_query: Query<(Entity, Option<&mut Transform>, Option<&mut Sprite>)>,
    item_query: Query<(
        Entity,
        Option<&Type>,
        &Location,
        Option<&Size>,
        Option<&Pawn>,
    )>,
    mut commands: Commands,
) {
    let mut done = Vec::new();
    for (entity_id, tyep_maybe, location, size_maybe, pawn_maybe) in item_query.iter() {
        // if the entity is in the world make sure it has a paen
        if let &Location::World { x, y } = location {
            let (width, height) = if let Some(&Size { width, height }) = size_maybe {
                (width as Number, height as Number)
            } else {
                (crate::CELL_SIZE, crate::CELL_SIZE)
            };
            let color = if let Some(Type(tyep)) = tyep_maybe {
                crate::type_to_color(tyep)
            } else {
                PINK.into()
            };
            let translate = Vec3 {
                x: x as f32,
                y: y as f32,
                z: 0.0,
            };
            // see if it already has pawn
            if let Some(&Pawn(pawn_id)) = pawn_maybe {
                // it has a pawn so try to get it
                if let Ok((_, transform_maybe, sprite_maybe)) = pawn_query.get_mut(pawn_id) {
                    done.push(pawn_id);
                    // the pawn had a transgorm os set the translaion
                    if let Some(mut transform) = transform_maybe {
                        transform.translation = translate;
                    } else {
                        // the pawn didn't have a translation so add one
                        commands.entity(pawn_id).insert(Transform {
                            translation: bevy::math::vec2(x as f32, y as f32).extend(0.0),
                            scale: Vec3::new(width as f32, height as f32, 1.0),
                            ..default()
                        });
                    }
                    // the pawn has a sprite so set it's color
                    if let Some(_sprite) = sprite_maybe {
                        // re don't update so we can highlight it when we hover
                        //sprite.color = color;
                    } else {
                        // the pawn didn't have a sprite so create one
                        commands
                            .entity(pawn_id)
                            .insert(Sprite { color, ..default() });
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
                    translation: bevy::math::vec2(x as f32, y as f32).extend(0.0),
                    scale: Vec3::new(width as f32, height as f32, 1.0),
                    ..default()
                };
                let sprite = Sprite { color, ..default() };
                let pawn_id = commands.spawn((sprite, transform)).id();
                commands.entity(entity_id).insert(Pawn(pawn_id));
            }
        }
    }
}
