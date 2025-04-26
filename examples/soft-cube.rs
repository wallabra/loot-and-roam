//! # Soft-body demonstration with a cube.
//!
//! Demonstrates Loot & Roam's point-based softbody physics system,
//! with a "sufficiently rigid" cube against a mildly bouncy floor
//! plane, of which a circle is visible.

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

// Demo is a modified variant of Bevy's 3D cube example '3d/3d_scene':
// https://github.com/bevyengine/bevy/blob/latest/examples/3d/3d_scene.rs

use std::f32::consts::TAU;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::PresentMode,
};
use loot_and_roam::{
    app::renderer::object::{DevCamera, ObjectRendererPlugin, PointAttach},
    common::physics::prelude::*,
};

/// Point netowrk snapping market component.
#[derive(Component)]
pub struct SnapToPointNet;

pub fn apply_example_systems(app: &mut App) {
    // Center cube on the average of its physics points, and orient it into the
    // point as a sort of cage.
    app.add_systems(
        Update,
        |mut query: Query<(&mut Transform, &PointNetwork), With<SnapToPointNet>>| {
            for (mut transform, network) in query.iter_mut() {
                if !network.points.is_empty() {
                    let len = network.points.len() as f32;
                    let avg: Vec3 = network
                        .points
                        .iter()
                        .map(|point| point.pos)
                        .fold(Vec3::ZERO, |acc, pos| acc + pos);
                    let avg = avg / len;

                    // since the first 8 vertices are all cube corner vertices,
                    // we can assume that they're orthogonal enough that any
                    // arbitrary pick within these bounds will allow for
                    // sufficient reorientation of the snapped cube mesh.

                    let front = network.points[0].pos.clone();
                    let up = network.points[2].pos.clone();
                    let up = (up - avg).normalize();

                    transform.translation = avg;
                    transform.look_at(front, up);

                    // the cube is facing the 'front' vertex now; we need to
                    // rotate it slightly so it aligns corner-wise rather than
                    // face-wise. (so it... "corners" the vertex? badum-tss!)
                    transform.rotate_local_x(TAU * 0.125);
                    transform.rotate_local_y(TAU * 0.125);
                } else {
                    panic!("Tried to reflect empty PointNetwork onto a Transform!");
                }
            }
        },
    );

    app.add_systems(Startup, setup);
}

/// Bevy setup system for the softbody cube demo.
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // -- circular base
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

    // -- light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // -- camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        DevCamera {
            move_speed: 5.0,
            rotate_sensitivity: 0.1,
            pitch: 0.0,
            yaw: 0.0,
            enabled: true,
        },
    ));
    // -- cube

    // create point & spring networks
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
    let springs = points.make_radially_connected_springs(
        spring_mode,
        1.5, /* max spring auto-connection range */
    );

    info!(
        "Cube has {} points and {} springs",
        points.points.len(),
        springs.springs.len()
    );

    // disturb point velocities for more interesting system stabiliztion observation
    points.points[3].vel.y += 1.5;
    points.points[5].vel.z += 2.0;

    // generate point network visualization as little children balls
    let children = (0..points.points.len())
        .map(|point_idx| {
            let child_point = commands
                .spawn((
                    PointAttach { point_idx },
                    Mesh3d(meshes.add(Sphere::new(0.04))),
                    MeshMaterial3d(materials.add(Color::srgb_u8(255, 255, 64))),
                    Transform::default(),
                ))
                .id();

            child_point
        })
        .collect::<Vec<_>>();

    // create cube entity
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
                force: Vec3::Y * -3.0,
            },
            SnapToPointNet,
            // CameraFocus::default(),
        ))
        .id();

    commands.entity(cube).add_children(&children);
}

fn main() {
    let mut app = App::new();

    // default plugin & main properties
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Loot & Roam Tech Demo - Soft Body Cube".into(),
            name: Some("bevy.loot-and-roam.techdemo.softbody".into()),
            present_mode: PresentMode::AutoNoVsync,
            ..default()
        }),
        ..default()
    }));

    // engine systems
    app.add_plugins((
        FrameTimeDiagnosticsPlugin::default(),
        BasicPhysicsPlugin,
        CollisionPlugin,
        ObjectRendererPlugin,
    ));

    // system registration
    apply_example_systems(&mut app);

    // logger
    app.add_plugins(LogDiagnosticsPlugin::default());

    app.run();
}
