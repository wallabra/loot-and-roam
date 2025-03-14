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

// -- development demo based on Bevy example '3d/3d_scene.rs'

use std::f32::consts::TAU;

use bevy::prelude::*;

use crate::common::physics::{Gravity, NormalSpring, PhysPoint, PointNetwork};

// [TODO] Please uncomment *only* implemented modules.
// pub mod renderer;
// pub mod audio;
// pub mod resource;
// pub mod input;
// pub mod ui;

/// Point netowrk snapping market component.
#[derive(Component)]
pub struct SnapToPointNet;

/// Camera target component.
#[derive(Component)]
pub struct CameraFocus {
    /// Focus priority, highest value is used to point camera at.
    prio: f32,
}

pub fn apply_app_systems(app: &mut App) {
    app.add_systems(
        Update,
        |mut query: Query<(&mut Transform, &PointNetwork), With<SnapToPointNet>>| {
            for (mut transform, network) in query.iter_mut() {
                if !network.points.is_empty() {
                    // Center Transform on average of all physics points
                    let len = network.points.len() as f32;
                    let avg: Vec3 = network
                        .points
                        .iter()
                        .map(|point| point.pos)
                        .fold(Vec3::ZERO, |acc, pos| acc + pos);
                    let avg = avg * 2.0 / len;

                    let front = network.points[0].pos.clone();
                    let up = network.points[(len * 0.5).floor() as usize].pos.clone();
                    let up = (up - avg).normalize();

                    transform.translation = avg;
                    transform.look_at(front, up);
                    transform.rotate_local_x(TAU * 0.125);
                    transform.rotate_local_y(TAU * 0.125);
                    transform.rotate_local_z(TAU * 0.125);
                } else {
                    panic!("Tried to reflect empty PointNetwork onto a Transform!");
                }
            }
        },
    );

    app.add_systems(Update, (debug_point_attach_snap,));

    app.add_systems(
        Update,
        |mut cam_query: Query<&mut Transform, With<Camera3d>>,
         focus_query: Query<(&CameraFocus, &Transform), Without<Camera3d>>| {
            let mut focus = focus_query.iter().collect::<Vec<_>>();

            if focus.is_empty() {
                return;
            }

            focus.sort_by(|a, b| a.0.prio.partial_cmp(&b.0.prio).unwrap());
            let focus = focus.last().unwrap().1;

            for mut cam_transform in cam_query.iter_mut() {
                cam_transform.look_at(focus.translation, Vec3::Y);
            }
        },
    );

    app.add_systems(Startup, setup);
}

// -- Show physics points for debugging purposes

/// Component attached to physics points when debugging
#[derive(Component)]
struct DebugPointAttach {
    point_idx: usize,
}

fn debug_point_attach_snap(
    mut query_child: Query<(&Parent, &mut Transform, &DebugPointAttach)>,
    query_parent: Query<&PointNetwork>,
) {
    for (parent, mut transform, attachment) in query_child.iter_mut() {
        let parent_points: &PointNetwork = query_parent.get(parent.get()).unwrap();

        let point = &parent_points.points[attachment.point_idx].pos;
        transform.translation = *point;
    }
}

/// Bevy setup system for the main Loot & Roam application.
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::Srgba(Color::WHITE.to_srgba().with_alpha(0.6)),
            alpha_mode: AlphaMode::Multiply,
            ..Default::default()
        })),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
            .with_translation(Vec3::new(0.0, -0.5, 0.0)),
    ));

    // cube
    let mut points = PointNetwork::from(
        [
            [-0.5, -0.5, -0.5],
            [-0.5, -0.5, 0.5],
            [-0.5, 0.5, -0.5],
            [-0.5, 0.5, 0.5],
            [0.5, -0.5, -0.5],
            [0.5, -0.5, 0.5],
            [0.5, 0.5, -0.5],
            [0.5, 0.5, 0.5],
        ]
        .map(|arr| PhysPoint::from_pos(Vec3::from(arr)))
        .into_iter(),
    );
    let springs = points.make_radially_connected_springs(
        crate::common::physics::SpringMode::Normal(NormalSpring { stiffness: 10.0 }),
        1.5, // bigger than sqrt(2) so should connect face vertices diagonally too
             // but smaller than cbrt(3) so should not connect fully opposite vertices
    );

    info!(
        "Cube has {} points and {} springs",
        points.points.len(),
        springs.springs.len()
    );

    // [NOTE] temporary test tug, or TTT for short :D
    points.points[3].vel.y += 3.0;
    points.points[3].vel.x += 1.0;

    let num_points = points.points.len();

    let cube = commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgba_u8(124, 144, 255, 140),
                alpha_mode: AlphaMode::Blend,
                ..Default::default()
            })),
            Transform::default(),
            points,
            springs,
            Gravity {
                // low grav for development purposes
                force: Vec3::Y * -0.2,
            },
            SnapToPointNet,
        ))
        .id();

    for point_idx in 0..num_points {
        let child_point = commands
            .spawn((
                DebugPointAttach { point_idx },
                Mesh3d(meshes.add(Sphere::new(0.04))),
                MeshMaterial3d(materials.add(Color::srgb_u8(255, 255, 64))),
                Transform::default(),
            ))
            .id();

        commands.entity(cube).add_child(child_point);
    }

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
