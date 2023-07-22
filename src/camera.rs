use bevy::prelude::*;


pub fn spawn_camera(
    mut commands: Commands
) {
    let mut camera = Camera2dBundle::default();
    camera.transform.translation = Vec3::new(
        4.* 32.,
        4. * 32.,
        camera.transform.translation.z
    );
    commands.spawn(camera);
}