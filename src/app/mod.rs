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

use crate::common::physics::{Gravity, NormalSpring, PhysPoint, PointNetwork, SpringMode};

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
#[derive(Component, Default)]
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
                    let avg = avg / len;

                    let front = network.points[0].pos.clone();
                    let up = network.points[(len * 0.333).floor() as usize].pos.clone();
                    let up = (up - avg).normalize();

                    transform.translation = avg;
                    transform.look_at(front, up);
                    transform.rotate_local_x(TAU * 0.125);
                    transform.rotate_local_y(TAU * 0.125);
                } else {
                    panic!("Tried to reflect empty PointNetwork onto a Transform!");
                }
            }
        },
    );

    app.add_systems(
        Update,
        (debug_point_attach_snap, floor_plane_collision_system),
    );

    app.add_systems(
        Update,
        |mut cam_query: Query<&mut Transform, With<Camera3d>>,
         focus_query: Query<(&CameraFocus, &Transform), Without<Camera3d>>| {
            let mut focus = focus_query.iter().collect::<Vec<_>>();

            if focus.is_empty() {
                return;
            }

            focus.sort_by(|a, b| b.0.prio.partial_cmp(&a.0.prio).unwrap());
            let focus = focus[0].1;

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
    query_parent: Query<(&PointNetwork, &GlobalTransform, &Transform), Without<DebugPointAttach>>,
) {
    for (parent, mut transform, attachment) in query_child.iter_mut() {
        let (parent_points, parent_global_transform, parent_transform) =
            query_parent.get(parent.get()).unwrap();

        transform.translation =
            parent_points.points[attachment.point_idx].pos - parent_global_transform.translation();
        transform.rotate_around(Vec3::ZERO, parent_transform.rotation.inverse());
    }
}

// Floor plane collisions
#[derive(Default, Component)]
pub struct FloorPlaneCollision {
    intercept_y: f32,
    restitution: f32,
    friction: f32,
}

pub fn floor_plane_collision_system(mut query: Query<(&mut PointNetwork, &FloorPlaneCollision)>) {
    for (mut points, collision) in query.iter_mut() {
        for point in &mut points.points {
            if point.pos.y < collision.intercept_y {
                point.pos.y = collision.intercept_y;
                point.vel.y *= -collision.restitution;

                let mut shift = point.vel * -collision.friction / point.mass;

                if shift.length_squared() > point.vel.length_squared() {
                    point.vel.x = 0.0;
                    point.vel.z = 0.0;
                } else {
                    shift.y = 0.0;
                    point.vel.x += shift.x;
                    point.vel.z += shift.z;
                }
            }
        }
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
            // cube corners
            [-0.5, -0.5, -0.5],
            [-0.5, -0.5, 0.5],
            [-0.5, 0.5, -0.5],
            [-0.5, 0.5, 0.5],
            [0.5, -0.5, -0.5],
            [0.5, -0.5, 0.5],
            [0.5, 0.5, -0.5],
            [0.5, 0.5, 0.5],
            // face centers
            [0.5, 0.0, 0.0],
            [0.0, 0.5, 0.0],
            [0.0, 0.0, 0.5],
            [-0.5, 0.0, 0.0],
            [0.0, -0.5, 0.0],
            [0.0, 0.0, -0.5],
        ]
        .map(|arr| PhysPoint::from_pos(Vec3::from(arr) + Vec3::Y))
        .into_iter(),
    );

    let spring_mode = SpringMode::Normal(NormalSpring { stiffness: 30.0 });
    let springs = points.make_radially_connected_springs(spring_mode, 1.5);
    // let springs = points.make_fully_connected_springs(spring_mode);

    info!(
        "Cube has {} points and {} springs",
        points.points.len(),
        springs.springs.len()
    );

    // [NOTE] temporary test tug, or TTT for short :D
    points.points[3].vel.y += 1.5;
    points.points[5].vel.z += 2.0;

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
            FloorPlaneCollision {
                restitution: 0.2,
                friction: 0.2,
                intercept_y: -0.5,
            },
            Gravity {
                // low grav for development purposes
                force: Vec3::Y * -0.1,
            },
            SnapToPointNet,
            CameraFocus::default(),
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
