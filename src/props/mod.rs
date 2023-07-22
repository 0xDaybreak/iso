use bevy::prelude::*;
use crate::board::components::Position;
use crate::pieces;
use crate::vectors::Vector2Int;


pub struct PropsPlugin;

impl Plugin for PropsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_prop);
    }
}

#[derive(Component)]
pub struct Prop;

fn spawn_prop(
    mut commands: Commands
) {
    commands.spawn((
        Prop,
        pieces::components::Occupier,
        Position { v: Vector2Int::new(4,5), pressed: Vector2Int::new(0,0)},
    ));
}
