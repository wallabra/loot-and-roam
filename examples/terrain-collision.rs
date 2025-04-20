//! 3D terrain-object collision demo.

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

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::PresentMode;
use loot_and_roam::app::renderer::object::PointAttach;
use loot_and_roam::common::physics::volume::VolumeCloneSpawner;
use loot_and_roam::common::prelude::*;
use loot_and_roam::common::terrain::buffer::TerrainBuffer;

/// Point netowrk snapping market component.
#[derive(Component)]
struct SnapToPointNet;

fn generate_terrain() -> TerrainBuffer {
    // initialize terrain generator
    let mut rng = rand::rng();

    let terragen = DefaultTerrainGeneratorBuilder::default()
        .noise(FractalNoise::random_octaves(
            10.0,
            10.0,
            4.try_into().unwrap(),
            &mut rng,
        ))
        .modulator(default_modulator())
        .modulation_params(ModulationParams {
            min_shore_distance: 4.0,
            max_shore_distance: 14.0,
            ..Default::default()
        })
        .center_points(vec![
            CenterPoint::new(Vec2::new(20.0, 35.0), 1.5),
            CenterPoint::new(Vec2::new(40.0, 20.0), 0.6),
        ])
        .resolution(6.0)
        .build()
        .unwrap();

    TerrainBuffer::generate(terragen, 0.3, 3.0, 80.0)
}

fn scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // get terrain mesh
    let terrain = generate_terrain();

    // spawn camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(200.0, 110.0, 200.0).looking_at(Vec3::Y * 10.0, Vec3::Y),
    ));

    // spawn light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 5000.0,
            range: 2000.0,
            ..default()
        },
        Transform::from_xyz(100.0, 300.0, -150.0),
    ));

    // spawn terrain mesh
    commands.spawn((
        terrain.as_bundle(&mut meshes),
        MeshMaterial3d(materials.add(Color::srgb_u8(80, 190, 45))),
        Transform::from_xyz(0.0, -40.0, 0.0),
    ));

    // spawn cubes
    for at in [
        [-0.2, 1.5, 1.0],
        [1.5, 36.5, 1.5],
        [-20.0, 60.0, -20.0],
        [-5.0, 80.0, 10.0],
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
                10.0,
                // cube_mesh.clone(),
                // cube_material.clone(),
                // point_mesh.clone(),
                // point_material.clone()
            )
        );
    }
}

fn spawn_cube(
    at: Vec3,
    commands: &mut Commands<'_, '_>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    size: f32,
    // cube_mesh: Handle<Mesh>,
    // cube_material: Handle<StandardMaterial>,
    // point_mesh: Handle<Mesh>,
    // point_material: Handle<StandardMaterial>,
) -> Entity {
    let cube_mesh = meshes.add(Cuboid::new(size, size, size));
    let cube_material = materials.add(StandardMaterial {
        base_color: Color::srgba_u8(124, 144, 255, 140),
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    });

    // create point & spring networks
    let points = PointNetwork::from(
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
        .map(|arr| PhysPoint::from_pos(at + Vec3::from(arr) * size))
        .into_iter(),
    );

    let spring_mode = SpringMode::Normal(NormalSpring { stiffness: 30.0 });
    let springs = points.make_radially_connected_springs(
        spring_mode,
        1.5 * size, /* max spring auto-connection range */
    );
    let volumes = VolumeCollection::at_every_point(
        &points,
        VolumeCloneSpawner::new(VolumeType::Sphere(SphereDef {
            radius: size * std::f32::consts::SQRT_2 / 2.0,
        })),
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

    // create water plane
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(1000.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgba_u8(190, 190, 255, 90),
            ..Default::default()
        })),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
            .with_translation(Vec3::new(0.0, -40.0, 0.0)),
    ));

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
                intercept_y: -80.0,
            },
            Gravity::default(),
            WaterPhysics {
                water_level: -60.0,

                // exaggerated for demonstrative purposes
                buoyancy_factor: 4.0,

                ..Default::default()
            },
            SnapToPointNet,
            //CameraFocus::default(),
        ))
        .id();

    commands.entity(cube).add_children(&children);

    cube
}

struct SnapToPointNetPlugin;

impl Plugin for SnapToPointNetPlugin {
    fn build(&self, app: &mut App) {
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
                    }
                }
            },
        );
    }
}

fn apply_example(app: &mut App) {
    app.add_systems(Startup, scene);
    app.add_plugins((SnapToPointNetPlugin,));
}

fn main() {
    let mut app = App::new();

    // image export

    // default plugin & main properties
    app.add_plugins((DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Loot & Roam Tech Demo - Terrain Renderer".into(),
            name: Some("bevy.loot-and-roam.techdemo.terrarender".into()),
            present_mode: PresentMode::AutoNoVsync,
            ..default()
        }),
        ..default()
    }),));

    apply_example(&mut app);

    // engine systems
    app.add_plugins((CommonPlugin, FrameTimeDiagnosticsPlugin));

    // logger
    app.add_plugins(LogDiagnosticsPlugin::default());

    app.run();
}
