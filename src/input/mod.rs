use bevy::prelude::*;
use crate::board::components::Position;
use crate::pieces::components::{Occupier, Piece};
use crate::player::Player;
use crate::vectors::Vector2Int;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_position);
    }
}

const DIR_KEY_MAPPING: [(KeyCode, Vector2Int); 4] = [
    (KeyCode::W, Vector2Int::RIGHT), (KeyCode::S, Vector2Int::LEFT),
    (KeyCode::A, Vector2Int::UP), (KeyCode::D, Vector2Int::DOWN),
];

fn player_position(
    keys:ResMut<Input<KeyCode>>,
    mut player_query: Query<&mut Position, (With<Player>, Without<Occupier>)>,
    occupier_query: Query<&Position, With<Occupier>>
) {
    let Ok(mut position) = player_query.get_single_mut() else { return };
    let mut target_position = position.v;
    for (key, dir) in DIR_KEY_MAPPING {
        if !keys.just_pressed(key) {continue;}
        target_position += dir;
        position.pressed = dir;
    }
    if occupier_query.iter().any(|occupier_pos| occupier_pos.v == target_position) {
        return;
    }

    position.v = target_position;
}