use bevy::prelude::*;
use bevy::prelude::system_adapter::new;
use bevy::render::render_resource::Texture;
use crate::board::components::{Position, Tile};
use crate::board::TILE_Z;
use crate::player::Player;
use crate::vectors::Vector2Int;


pub fn spawn_tile_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Position), With<Tile>>,
    query_piece: Query<(Entity, &Position), With<Player>>,
    asset: Res<AssetServer>
) {
    let mut pos: Option<Vector2Int>  = None;
    for (_, position) in query_piece.iter() {
        pos = Some(position.v);
    }
    if pos.is_none() {
        return;
    }
    let pos = get_surrounding_coordinates(&pos.unwrap());



    for (entity, position) in query.iter() {
        let v = super::get_world_position(&position, TILE_Z);

        let mut sprite: Handle<Image> = asset.load("tile-export.png");
        if pos.contains(&position.v) {
            sprite = asset.load("bluetile.png");
        }

        commands.entity(entity)
            .insert(
                SpriteBundle{
                    transform: Transform::from_translation(v),
                    texture: sprite,
                    ..default()
                }
            );
    }
}

fn get_surrounding_coordinates(v: &Vector2Int) -> Vec<Vector2Int> {
    let mut res = Vec::new();

    // Define relative movements for neighbors (up, down, left, right)
    let movements = vec![Vector2Int::new(-2, -1), Vector2Int::new(-1, 0), Vector2Int::new(0, -1),
                         Vector2Int::new(-1, -2), Vector2Int::new(0,0), Vector2Int::new(0,-2), Vector2Int::new(-2,0), Vector2Int::new(-2,-2)];

    // Iterate through relative movements to find neighbors
    for movement in &movements {
        let neighbor = *v + *movement;
        res.push(neighbor);
    }

    res
}