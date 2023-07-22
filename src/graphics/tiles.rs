use bevy::prelude::*;
use crate::board::components::{Position, Tile};
use crate::board::TILE_Z;


pub fn spawn_tile_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Position), Added<Tile>>,
    asset: Res<AssetServer>
) {
    for (entity, position) in query.iter() {
        let v = super::get_world_position(&position, TILE_Z);
        commands.entity(entity)
            .insert(
                SpriteBundle{
                    transform: Transform::from_translation(v),
                    texture: asset.load("tile-export.png"),
                    ..default()
                }
            );
    }
}