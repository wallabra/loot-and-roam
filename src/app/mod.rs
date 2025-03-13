//! # The official Loot & Roam client.
//!
//! Contains code for displaying the game, interacting with user input,
//! loading client-side assets, handling in-game audio, and so on.

// Written by:
// * Gustavo Ramos Rehermann <rehermann6046@gmail.com>
//
// (c)2025 GameCircular. Under the Cooperative Non-Violent Public License.
//
// Loot & Roam is non-violent software: you can use, redistribute,
// and/or modify it under the terms of the CNPLv6+ as found
// in the LICENSE file in the source code root directory or
// at <https://git.pixie.town/thufie/CNPL>.
//
// Loot & Roam comes with ABSOLUTELY NO WARRANTY, to the extent
// permitted by applicable law.  See the CNPL for details.

use std::f32::consts::TAU;

use bevy::prelude::*;

use crate::common::physics::{Gravity, NormalSpring, PhysPoint, PointNetwork};

// [TODO] Please uncomment *only* implemented modules.
// pub mod renderer;
// pub mod audio;
// pub mod resource;
// pub mod input;
// pub mod ui;

/// Temporary tick update to check performance
pub fn rotate(mut cubes: Query<(&mut Transform, &Rotatable)>, timer: Res<Time>) {
    for (mut transform, cube) in &mut cubes {
        // The speed is first multiplied by TAU which is a full rotation (360deg) in radians,
        // and then multiplied by delta_secs which is the time that passed last frame.
        // In other words. Speed is equal to the amount of rotations per second.
        transform.rotate_y(cube.speed * TAU * timer.delta_secs());
    }
}

/// Cube rotation component.
#[derive(Component)]
pub struct Rotatable {
    speed: f32,
}

pub fn apply_app_systems(app: &mut App) {
    app.add_systems(
        Update,
        |mut query: Query<(&mut Transform, &PointNetwork)>| {
            for (mut transform, network) in query.iter_mut() {
                if network.points.len() > 0 {
                    // Center Transform on average of all physics points
                    let avg: ultraviolet::Vec3 = network
                        .points
                        .iter()
                        .map(|point| point.pos)
                        .reduce(|acc, vec| acc + vec)
                        .unwrap()
                        / network.points.len() as f32;
                    transform.translation.x = avg.x;
                    transform.translation.y = avg.y;
                    transform.translation.z = avg.z;
                } else {
                    panic!("Tried to reflect empty PointNetwork onto a Transform!");
                }
            }
        },
    );
}

/// Bevy setup system for the main Loot & Roam application.
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // -- stub copied from the Bevy example '3d/3d_scene.rs'
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
            .with_translation(Vec3::new(0.0, -0.5, 0.0)),
    ));

    // cube
    let points = PointNetwork::from(
        [
            [-0.5, 0.5, -0.5],
            [-0.5, 0.5, 0.5],
            [-0.5, 1.5, -0.5],
            [-0.5, 1.5, 0.5],
            [0.5, 0.5, -0.5],
            [0.5, 0.5, 0.5],
            [0.5, 1.5, -0.5],
            [0.5, 1.5, 0.5],
        ]
        .map(|arr| PhysPoint::from_pos(ultraviolet::Vec3::from(arr)))
        .into_iter(),
    );
    let springs = points.make_radially_connected_springs(
        crate::common::physics::SpringMode::Normal(NormalSpring { stiffness: 3.0 }),
        1.0,
    );

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
        Rotatable { speed: 0.3 },
        points,
        springs,
        Gravity {
            // low grav for development purposes
            force: ultraviolet::Vec3::unit_y() * -0.5,
        },
    ));

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
