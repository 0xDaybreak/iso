use bevy::prelude::*;

#[derive(Component)]
pub struct Piece {
    pub kind: String,
}

#[derive(Component)]
pub struct Occupier;
