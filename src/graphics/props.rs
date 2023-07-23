use std::borrow::Borrow;
use std::collections::HashSet;
use bevy::prelude::*;
use bevy::render::render_resource::Texture;
use crate::board::components::Position;
use crate::board::{PIECE_Z, PROP_Z};
use crate::pieces::components::Piece;
use crate::player::Player;
use crate::props::Prop;
use crate::vectors::Vector2Int;

pub fn update_prop_z(
    mut commands: Commands,
    query: Query<(Entity, &Position), With<Prop>>,
    query_piece: Query<(Entity, &Position), With<Player>>,
    assets: Res<AssetServer>,
) {

    let mut prop_z = 11.;
    let mut pos: Option<Vector2Int>  = None;
    for (_, position) in query_piece.iter() {
        pos = Some(position.v);
    }
    if pos.is_none() {
        return;
    }
    let pos = get_surrounding_coordinates(&pos.unwrap());

    for (entity, position) in query.iter() {

        let sprite = assets.load("gard.png");
        let mut v = super::get_world_position(&position, PROP_Z);

        if pos.contains(&position.v) {
            prop_z += 2.;
            v = super::get_world_position(&position, prop_z);
        }
       println!("{}", prop_z);


        commands.entity(entity).insert((
            SpriteBundle {
                texture: sprite,
                transform: Transform::from_translation(v),
                ..Default::default()
            }
        ));
    }
}

fn get_surrounding_coordinates(v: &Vector2Int) -> Vec<Vector2Int> {
    let mut res = Vec::new();

    let movements = vec![Vector2Int::new(-1, 0),Vector2Int::new(0, -1), Vector2Int::new(-1, -1), Vector2Int::new(-2, -1)];

    for movement in &movements {
        let neighbor = *v + *movement;
        res.push(neighbor);
    }
    res
}