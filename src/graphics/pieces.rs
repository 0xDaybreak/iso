use std::ops::Deref;
use bevy::prelude::*;
use crate::board::components::Position;
use crate::board::PIECE_Z;
use crate::graphics::{PIECE_SPEED, POSITION_TOLERANCE};
use crate::pieces::components::{Occupier, Piece};
use crate::props::Prop;


pub fn spawn_piece_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Position, &Piece), Added<Piece>>,
    assets: Res<AssetServer>,
) {
    for (entity, position, piece) in query.iter() {
        let v = super::get_world_position(&position, PIECE_Z);
        let sprite_idx = match piece.kind.as_str() {
            "Player" => 1,
            _ => 63
        };

        let sprite = assets.load("playerS.png");
        commands.entity(entity).insert(
            SpriteBundle{
                texture: sprite,
                transform: Transform::from_translation(v),
                ..default()
            }
        );
    }
}


pub fn update_piece_position(
    mut commands: Commands,
    mut query: Query<(Entity, &Position, &mut Transform), With <Piece>>,
    mut prop_query: Query<(Entity, &Transform, With<Prop>), Without<Piece>>,
    time: Res<Time>,
    assets: Res<AssetServer>,
) {
    for (entity, position, mut transform) in query.iter_mut() {
        let mut target = super::get_world_position(&position, PIECE_Z);
        for (prop_entity, prop_transform, prop) in prop_query.iter_mut() {
            let prop_z = prop_transform.translation.z;

            if target.z > prop_z {
                target.z = target.z - 2.0;
            }
        }

        let d = (target - transform.translation).length();

        if d > POSITION_TOLERANCE {
            transform.translation = transform.translation.lerp(
                target,
                PIECE_SPEED * time.delta_seconds(),
            );

            let sprite_x = match position.pressed.x {
                -1 => assets.load("playerS.png"),
                1 => assets.load("playerW.png"),
                _ => assets.load("playerS.png"),
            };

            let sprite_y = match position.pressed.y {
                -1 => assets.load("playerD.png"),
                1 => assets.load("playerA.png"),
                _ => assets.load("playerS.png"),
            };

            let sprite = if position.pressed.y != 0 {
                sprite_y
            } else {
                sprite_x
            };
            commands.entity(entity).insert(SpriteBundle {
                texture: sprite,
                transform: *transform,
                ..Default::default()
            });
        } else {
            transform.translation = target;
        }
    }
}