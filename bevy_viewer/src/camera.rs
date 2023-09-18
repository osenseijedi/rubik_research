use bevy::app::AppExit;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;


/// Tags an entity as capable of orbiting.
#[derive(Component)]
pub(crate) struct OrbitCamera {
    /// The "focus point" to orbit around
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        OrbitCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            upside_down: false,
        }
    }
}

pub(crate) fn setup_camera(commands: &mut Commands, translation: Vec3) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        OrbitCamera {
            radius: translation.length(),
            ..Default::default()
        },
    ));
}

/// Zoom with scroll wheel, orbit with right mouse click.
pub(crate) fn orbit_camera_update(
    mut windows: Query<&mut Window>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>,
    input_keyboard: Res<Input<KeyCode>>,
    mut camera_query: Query<(&mut OrbitCamera, &mut Transform)>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    let orbit_button = MouseButton::Right;

    let mut rotation_move = Vec2::ZERO;
    let mut scroll = 0.0;
    let mut orbit_button_changed = false;

    if input_keyboard.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit);
    }

    if input_mouse.pressed(orbit_button) {
        for ev in ev_motion.iter() {
            rotation_move += ev.delta;
        }
    }
    for ev in ev_scroll.iter() {
        scroll += ev.y;
    }
    if input_mouse.just_released(orbit_button) || input_mouse.just_pressed(orbit_button) {
        orbit_button_changed = true;
    }

    let window = windows.single_mut();
    let window_width = window.width();
    let window_height = window.height();


    for (mut orbit, mut transform) in camera_query.iter_mut() {
        if orbit_button_changed {
            let up = transform.rotation * Vec3::Y;
            orbit.upside_down = up.y <= 0.0;
        }

        let mut any = false;
        if rotation_move.length_squared() > 0.0 {
            any = true;


            let delta_x = {
                let delta = rotation_move.x / window_width * std::f32::consts::PI;// * 2.0;
                if orbit.upside_down { -delta } else { delta }
            };
            let delta_y = rotation_move.y / window_height * std::f32::consts::PI;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);

            transform.rotation = yaw * transform.rotation; // rotate around global y axis
            transform.rotation = transform.rotation * pitch; // rotate around local x axis
        } else if scroll.abs() > 0.0 {
            any = true;
            orbit.radius -= scroll * orbit.radius * 0.2;
            // dont allow zoom to reach zero or you get stuck
            orbit.radius = f32::max(orbit.radius, 0.05);
        }

        if any {
            // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
            // parent = x and y rotation
            // child = z-offset
            let rot_matrix = Mat3::from_quat(transform.rotation);
            transform.translation = orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, orbit.radius));
        }
    }

    // consume any remaining events, so they don't pile up if we don't need them
    // (and also to avoid Bevy warning us about not checking events every frame update)
    ev_motion.clear();
}
