use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;

#[derive(Component)]
pub struct ChangeColor;

pub fn setup_mesh(commands: &mut Commands,
                  meshes: &mut ResMut<Assets<Mesh>>,
                  materials: &mut ResMut<Assets<StandardMaterial>>) {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let color = Color::GREEN;

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[0., 0., 0.], [1., 1., 0.], [0., 1., 0.],
             [1., 0., 0.], [1., 1., 0.], [0., 0., 0.],
        ],
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![
        [0., 0., 1.], [0., 0., 1.], [0., 0., 1.],
        [0., 0., 1.], [0., 0., 1.], [0., 0., 1.],
    ]);

    commands.spawn((PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(color.into()),
        ..default()
    }, ChangeColor));

}

pub fn update_mesh(
    input_keyboard: Res<Input<KeyCode>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut change_color: Query<(&mut ChangeColor, &Handle<StandardMaterial>)>,
) {
    for (mut _cc, mat_handle) in change_color.iter_mut() {
        if input_keyboard.just_pressed(KeyCode::A) {
            let mat = materials.get_mut(mat_handle).unwrap();
            mat.base_color = Color::RED;
        }
        if input_keyboard.just_pressed(KeyCode::S) {
            let mat = materials.get_mut(mat_handle).unwrap();
            mat.base_color = Color::BLUE;
        }
    }
}