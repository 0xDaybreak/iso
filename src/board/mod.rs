use std::collections::HashMap;
use bevy::prelude::*;
use crate::vectors::Vector2Int;

pub mod components;
mod system;


pub const TILE_WIDTH: f32 = 64.;
pub const TILE_HEIGHT: f32 = 32.;
pub const TILE_Z: f32 = 0.;
pub const PIECE_Z: f32 = 12.;
pub const PROP_Z: f32 = 11.;


pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentBoard>()
            .add_systems(Startup, system::spawn_map);
    }
}

#[derive(Default, Resource)]
pub struct CurrentBoard {
    pub tiles: HashMap<Vector2Int, Entity>
}