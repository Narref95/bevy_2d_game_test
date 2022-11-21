use bevy::prelude::*;

use crate::components::*;

pub struct LookAtCameraPlugin;

impl Plugin for LookAtCameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(look_at_camera);
    }
}

fn look_at_camera(
    mut query_comps: Query<&mut Transform, (With<LookAtCamera>, Without<MainCamera>)>,
    mut query_cam: Query<&mut Transform, (With<MainCamera>, Without<Player>)>
){
    if let Ok(mut comp_transform) = query_comps.get_single_mut() {
        if let Ok(cam_transform) = query_cam.get_single_mut() {
            comp_transform.rotation = cam_transform.rotation;
        }
    }
}