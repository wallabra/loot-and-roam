//! 3D terrain meshing test demo.

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
use loot_and_roam::common::prelude::*;
use loot_and_roam::common::terrain::buffer::TerrainBuffer;

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

    TerrainBuffer::generate(terragen, 0.3, 3.0, 100.0)
}

fn scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // get terrain mesh
    let terrain = generate_terrain();
    println!(
        "Terrain buffer has dimensions {}x{} and {} tris",
        terrain.get_vert_width(),
        terrain.get_vert_height(),
        terrain.get_num_tris()
    );

    // spawn camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(700.0, 200.0, 500.0).looking_at(Vec3::Y * 30.0, Vec3::Y),
    ));

    // spawn light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            range: 50000.0,
            intensity: 10000.0,
            ..default()
        },
        Transform::from_xyz(30.0, 1000.0, -10.0),
    ));

    // spawn water level plane
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(1000.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgba_u8(20, 60, 255, 100),
            ..Default::default()
        })),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));

    // spawn terrain mesh
    commands.spawn((
        terrain.as_bundle(&mut meshes),
        MeshMaterial3d(materials.add(Color::srgb_u8(32, 140, 32))),
        Transform::default(),
    ));
}

fn apply_example(app: &mut App) {
    app.add_systems(Startup, scene);
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
    app.add_plugins((FrameTimeDiagnosticsPlugin,));

    // logger
    app.add_plugins(LogDiagnosticsPlugin::default());

    app.run();
}
