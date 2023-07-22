mod tiles;
mod pieces;
mod props;

use bevy::prelude::*;
use crate::board::components::Position;
use crate::board::{TILE_HEIGHT, TILE_WIDTH};

pub const PIECE_SPEED: f32 = 8.;
pub const POSITION_TOLERANCE: f32 = 0.1;

fn get_world_position(position: &Position, z: f32) -> Vec3 {
    let x = (position.v.x - position.v.y) as f32 * TILE_WIDTH / 2.0;
    let y = (position.v.x + position.v.y) as f32 * TILE_HEIGHT / 2.0;
    Vec3::new(x, y, z)
}


fn get_adjusted_world_position(position: &Position, z: f32) -> Vec3 {
    let x = (position.v.x - position.v.y) as f32 * TILE_WIDTH / 2.0;
    let y = (position.v.x + position.v.y) as f32 * TILE_HEIGHT / 2.0 + TILE_HEIGHT/2.;
    Vec3::new(x, y, z)
}


pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app:&mut App) {
        app.add_systems(Update, (tiles::spawn_tile_renderer,pieces::spawn_piece_renderer,
                                  props::spawn_prop_renderer, pieces::update_piece_position,));
    }
}
