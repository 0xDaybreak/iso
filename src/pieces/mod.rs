use bevy::prelude::*;
use crate::board::components::Position;
use crate::vectors::Vector2Int;

pub mod components;

pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,spawn_npcs);
    }
}

fn spawn_npcs (
    mut commands: Commands,
) {
    commands.spawn((
        components::Piece {kind: "NPC".to_string()},
        components::Occupier,
        Position {v:Vector2Int::new(7,2), pressed: Default::default() },
        ));
}

