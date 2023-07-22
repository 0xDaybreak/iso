use std::collections::HashMap;
use bevy::prelude::*;
use crate::board::components::{Position, Tile};
use crate::board::{CurrentBoard, TILE_HEIGHT, TILE_WIDTH};
use crate::vectors::Vector2Int;


pub fn spawn_map(
    mut commands: Commands,
    mut current: ResMut<CurrentBoard>
)  {
    current.tiles = HashMap::new();
    for x in 0..10 {
        for y in 0..10 {

            let v = Vector2Int::new(x as i32, y as i32);
            let tile = commands.spawn((
                Position { v, pressed: Vector2Int::new(0,0) },
                Tile,
            ))
                .id();
            current.tiles.insert(v, tile);
        }
    }
}