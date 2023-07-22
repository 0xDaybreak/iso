use bevy::prelude::*;
use crate::board::components::Position;
use crate::board::{PIECE_Z, PROP_Z};
use crate::props::Prop;

pub fn spawn_prop_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Position, &Prop), Added<Prop>>,
    assets: Res<AssetServer>,
) {
    for (entity, position, prop) in query.iter() {
        let v = super::get_adjusted_world_position(&position, PROP_Z);

        let sprite = assets.load("tree2.png");

        commands.entity(entity).insert(
            SpriteBundle {
                texture: sprite,
                transform: Transform::from_translation(v),
                ..default()
            }
        );
    }
}