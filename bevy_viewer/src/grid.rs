use bevy::prelude::*;
use bevy_polyline::prelude::*;

pub fn setup_grid(
    mut commands: Commands,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
) {
    let min = -10;
    let max = 10;
    let step = 1;

    let mut vertices = Vec::new();

    for i in (min..max).step_by(2 * step) {
        let i = i as f32;
        let min = min as f32;
        let max = max as f32;
        let step = step as f32;

        vertices.push(Vec3::new(i, 0.0, min));
        vertices.push(Vec3::new(i, 0.0, max));
        vertices.push(Vec3::new(i + step, 0.0, max));
        vertices.push(Vec3::new(i + step, 0.0, min));
    }

    for j in (min..max).step_by(2 * step) {
        let j = j as f32;
        let min = min as f32;
        let max = max as f32;
        let step = step as f32;

        vertices.push(Vec3::new(min, 0.0, j));
        vertices.push(Vec3::new(min, 0.0, j + step));
        vertices.push(Vec3::new(max, 0.0, j + step));
        vertices.push(Vec3::new(max, 0.0, j + 2.0 * step));
    }

    commands.spawn(PolylineBundle {
        polyline: polylines.add(Polyline { vertices }),
        material: polyline_materials.add(PolylineMaterial {
            width: 2.0,
            color: Color::WHITE,
            perspective: false,
            ..Default::default()
        }),
        ..Default::default()
    });
}
