use bevy::prelude::*;
use bevy_polyline::PolylinePlugin;

use polyhedron::Polyhedron;

use camera::*;
use grid::*;
use mesh::*;
use light::setup_light;

mod grid;
mod camera;
mod light;
mod mesh;

type RubikDefinition = polyhedron::definition_cube_2x2::Cube2x2Definition;
// type RubikDefinition = polyhedron::definition_tetrahedron_inflated_3x3::TetrahedronInflated3x3Definition;

pub fn start_bevy() {

    fn setup_scene(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
        let mut polyhedron = Polyhedron::create_polyhedron(RubikDefinition::new());
        setup_polyhedron_mesh(&mut commands, &mut meshes, &mut materials, polyhedron);

        let translation = Vec3::new(-2.0, 2.5, 5.0);
        setup_light(&mut commands, translation.clone());
        setup_camera(&mut commands, translation);
    }

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PolylinePlugin)
        .add_systems(Startup, (setup_scene, setup_grid))
        .add_systems(Update, (orbit_camera_update))
        .add_systems(Update, (update_mesh))
        .run();
}