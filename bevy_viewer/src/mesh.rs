use std::collections::HashMap;
use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;

use polyhedron::Polyhedron;
use polyhedron::polyhedron::PolyhedronDefinition;
use polyhedron::Quad;
use crate::RubikDefinition;

#[derive(Component)]
pub struct PolyhedronMesh<T>
    where T: PolyhedronDefinition {
    polyhedron: Polyhedron<T>,
}

pub fn setup_polyhedron_mesh(commands: &mut Commands,
                             meshes: &mut ResMut<Assets<Mesh>>,
                             materials: &mut ResMut<Assets<StandardMaterial>>,
                             polyhedron: Polyhedron<RubikDefinition>) {
    let positions: Vec<usize> = polyhedron.get_positions();
    let polyhedron_meshes_by_position_name: HashMap<usize, Quad> = polyhedron.get_meshes();

    for position in positions {
        let q = polyhedron_meshes_by_position_name.get(&position).unwrap();
        let color = convert_color(polyhedron.get_color(position));
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![
                q[0], q[1], q[2],
                q[0], q[2], q[3],
            ]);
        mesh.compute_flat_normals();

        commands.spawn(
            PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(color.into()),
                ..default()
            });
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let offset = 10f32;

    let q =
        [[offset + 0., offset + 0., offset + 0.],
            [offset + 1., offset + 0., offset + 0.],
            [offset + 1., offset + 1., offset + 0.],
            [offset + 0., offset + 1., offset + 0.]];

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![q[0], q[2], q[1], q[0], q[2], q[3]],
    );
    mesh.compute_flat_normals();

    commands.spawn((PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(Color::GREEN.into()),
        ..default()
    }, PolyhedronMesh { polyhedron }));
}

fn convert_color(colorful_color: )->Color{

    todo!()
}

pub fn update_mesh(
    input_keyboard: Res<Input<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut polyhedron_query: Query<(&mut PolyhedronMesh<RubikDefinition>, &Handle<StandardMaterial>)>,
    mut mesh_query: Query<&Handle<StandardMaterial>>,
) {
    for (mut polyhedron, mat_handle) in polyhedron_query.iter_mut() {
        if input_keyboard.just_pressed(KeyCode::P) {
            polyhedron.polyhedron.print_polyhedron();
        }
        if input_keyboard.just_pressed(KeyCode::S) {
            let mat = materials.get_mut(mat_handle).unwrap();
            mat.base_color = Color::BLUE;
        }
    }

    if input_keyboard.just_pressed(KeyCode::A) {
        println!("A");
        let mesh_handle = mesh_query.get_single();

        match mesh_handle {
            Ok(mesh_handle) => {}
            Err(_) => { println!("couldn't find"); }
        }
    }
}