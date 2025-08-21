//! # Collision system demonstration between soft-body cubes
//!
//! Demonstrates Loot & Roam's [VolumeCollection] and related collision system,
//! spawning multiple soft-body cubes with simple volumes on every physics
//! point, and allowing them to collide with gravity.

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

use std::f32::consts::{SQRT_2, TAU};

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    math::FloatOrd,
    prelude::*,
    render::{
        RenderPlugin,
        camera::{ImageRenderTarget, RenderTarget},
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
    },
    window::PresentMode,
};
use bevy_image_export::{ImageExport, ImageExportPlugin, ImageExportSettings, ImageExportSource};
use loot_and_roam::{
    app::renderer::object::{ObjectRendererPlugin, PointAttach},
    common::physics::{prelude::*, volume::VolumeCloneSpawner},
};

/// Point netowrk snapping market component.
#[derive(Component)]
struct SnapToPointNet;

fn apply_example_systems(app: &mut App) {
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

// Resolution for exporting demo images.
const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

/// Bevy setup system for the softbody cube collision demo.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut export_sources: ResMut<Assets<ImageExportSource>>,
) {
    // output texture for image sequence rendering
    let output_texture_handle = {
        let size = Extent3d {
            width: WIDTH,
            height: HEIGHT,
            ..default()
        };
        let mut export_texture = Image {
            texture_descriptor: TextureDescriptor {
                label: None,
                size,
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba8UnormSrgb,
                mip_level_count: 1,
                sample_count: 1,
                usage: TextureUsages::COPY_DST
                    | TextureUsages::COPY_SRC
                    | TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            },
            ..default()
        };
        export_texture.resize(size);

        images.add(export_texture)
    };

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

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // camera
    commands
        .spawn((
            Camera3d::default(),
            Transform::from_xyz(-5.0, 9.0, 18.0).looking_at(Vec3::Y * 0.5, Vec3::Y),
        ))
        .with_child((
            Camera3d::default(),
            Camera {
                // Connect the output texture to a camera as a RenderTarget.
                target: RenderTarget::Image(ImageRenderTarget {
                    handle: output_texture_handle.clone(),
                    scale_factor: FloatOrd(1.0),
                }),
                ..default()
            },
        ));

    // cubes
    for at in [
        [-0.2, 1.5, 1.0],
        [-0.4, 3.5, -0.5],
        [0.5, 6.25, 0.5],
        [1.5, 12.5, 1.5],
    ]
    .map(|arr| Vec3::from_array(arr))
    {
        println!(
            "cube spawned: {:?}",
            spawn_cube(
                at,
                &mut commands,
                &mut meshes,
                &mut materials,
                // cube_mesh.clone(),
                // cube_material.clone(),
                // point_mesh.clone(),
                // point_material.clone()
            )
        );
    }

    // start image exportation
    commands.spawn((
        ImageExport(export_sources.add(ImageExportSource(output_texture_handle.clone()))),
        ImageExportSettings {
            // Frames will be saved to "./out/soft-cube-collision/[#####].png"
            // [NOTE] update output dir when grafting this code onto other examples
            output_dir: "out/soft-cube-collision/".into(),

            // Choose "exr" for HDR renders (requires feature on crate bevy_image_export)
            extension: "png".into(),
        },
    ));
}

fn spawn_cube(
    at: Vec3,
    commands: &mut Commands<'_, '_>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    // cube_mesh: Handle<Mesh>,
    // cube_material: Handle<StandardMaterial>,
    // point_mesh: Handle<Mesh>,
    // point_material: Handle<StandardMaterial>,
) -> Entity {
    let cube_mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let cube_material = materials.add(StandardMaterial {
        base_color: Color::srgba_u8(124, 144, 255, 140),
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    });

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
        .map(|arr| PhysPoint::from_pos(at + Vec3::from(arr)))
        .into_iter(),
    );

    points.apply_angular_impulse(Vec3::Z * 30.0); // spawn spinning cubes

    let spring_mode = SpringMode::Normal(NormalSpring { stiffness: 30.0 });
    let springs = points.make_radially_connected_springs(
        spring_mode,
        1.5, /* max spring auto-connection range */
    );
    let volumes = VolumeCollection::at_every_point(
        &points,
        VolumeCloneSpawner::new(VolumeType::Sphere(SphereDef {
            radius: SQRT_2 / 2.0,
        })),
    );

    info!(
        "Cube has {} points, {} springs, {} volumes",
        points.points.len(),
        springs.springs.len(),
        volumes.volumes.len(),
    );

    // generate point network visualization as little children balls
    let children = (0..points.points.len())
        .map(|point_idx| {
            let point_mesh = meshes.add(Sphere::new(0.05));
            let point_material = materials.add(StandardMaterial {
                base_color: Color::srgba_u8(255, 255, 48, 200),
                alpha_mode: AlphaMode::Blend,
                ..Default::default()
            });

            let child_point = commands
                .spawn((
                    PointAttach { point_idx },
                    Mesh3d(point_mesh),
                    MeshMaterial3d(point_material),
                    Transform::default(),
                ))
                .id();

            child_point
        })
        .collect::<Vec<_>>();

    // create cube entity
    let cube = commands
        .spawn((
            Mesh3d(cube_mesh),
            MeshMaterial3d(cube_material),
            Transform::default(),
            points,
            springs,
            volumes,
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
            //CameraFocus::default(),
        ))
        .id();

    commands.entity(cube).add_children(&children);

    cube
}

fn main() {
    let mut app = App::new();

    // image export
    let export_plugin = ImageExportPlugin::default();
    let export_threads = export_plugin.threads.clone();

    // default plugin & main properties
    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Loot & Roam Tech Demo - Soft Body Cube".into(),
                    name: Some("bevy.loot-and-roam.techdemo.softbody".into()),
                    present_mode: PresentMode::AutoNoVsync,
                    ..default()
                }),
                ..default()
            })
            .set(RenderPlugin {
                synchronous_pipeline_compilation: true,
                ..default()
            }),
        export_plugin,
    ));

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

    // block till image sequence exportation is done
    export_threads.finish();

    // command to render to video:
    // $ ffmpeg -r 60 -i out/soft-cube-collision/%05d.png -vcodec libx264 -crf 25 -pix_fmt yuv420p out/soft-cube-collision.mp4
    // command to reset demo recordings:
    // $ rm -r out/
}
