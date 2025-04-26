use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerCamera;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera3d::default(), PlayerCamera));
}

pub fn camera_movement(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<PlayerCamera>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for mut transform in query.iter_mut() {
        let mut move_direction = Vec3::ZERO;
        let speed = 5.0;

        // Basic WASD movement and space/shift for vertical movement
        if keyboard_input.pressed(KeyCode::KeyW) {
            move_direction.z -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            move_direction.z += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            move_direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            move_direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Space) {
            move_direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ShiftLeft) {
            move_direction.y -= 1.0;
        }

        transform.translation += move_direction * speed * time.delta_secs();
    }
}
