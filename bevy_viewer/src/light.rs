use bevy::prelude::*;
use crate::camera::OrbitCamera;

pub fn setup_light(commands: &mut Commands, translation: Vec3) {
    commands.spawn((
        PointLightBundle {
            transform: Transform::from_translation(translation.clone()),
            ..Default::default()
        }, OrbitCamera {
            radius: translation.length(),
            ..Default::default()
        }
    ));
}