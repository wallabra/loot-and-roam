//! # Construct test: cube spitting watchtower
//!
//! Demonstrates Loot & Roam's water physics.

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

use itertools::Itertools;
use rand::distr::Distribution;
use std::f32::consts::SQRT_2;

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
use derive_builder::Builder;
use loot_and_roam::prelude::*;
use rand::distr::Uniform;

/// Cube spitter part component.
#[derive(Component, Debug)]
pub struct CubeSpitter {
    interval: f32,
    vel: Vec3,
    spawn_offset: Vec3,
    cooldown: f32,
}

impl CubeSpitter {
    fn spit_cube(
        &mut self,
        pos: Vec3,
        commands: &mut Commands<'_, '_>,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Option<Entity> {
        if self.cooldown > 0.0 {
            return None;
        }
        let cube = spawn_cube(pos + self.spawn_offset, commands, meshes, materials);
        let vel = self.vel;

        commands
            .entity(cube)
            .entry::<PointNetwork>()
            .and_modify(move |mut points| points.apply_instant_force(vel));

        // TODO: make cube spin randomly, for dramatic effect!

        self.cooldown = self.interval;

        Some(cube)
    }

    pub fn tick_cooldown(&mut self, delta_time_secs: f32) {
        self.cooldown = (self.cooldown - delta_time_secs).max(0.0_f32);
    }

    pub fn auto_spit(
        &mut self,
        this_entity: Entity,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        transform_query: Query<&GlobalTransform>,
    ) {
        let pos = transform_query.get(this_entity).unwrap().translation();
        self.spit_cube(pos, commands, meshes, materials);
    }

    pub fn check_auto_spit(
        &mut self,
        this_entity: Entity,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        transform_query: Query<&GlobalTransform>,
    ) {
        if self.cooldown <= 0.0 {
            self.auto_spit(this_entity, commands, meshes, materials, transform_query);
        }
    }

    pub fn new(interval: f32, vel: Vec3) -> Self {
        Self {
            interval,
            vel,
            spawn_offset: Vec3::ZERO,
            cooldown: 0.0,
        }
    }

    pub fn new_with_spawn_offset(interval: f32, vel: Vec3, spawn_offset: Vec3) -> Self {
        Self {
            interval,
            vel,
            spawn_offset,
            cooldown: 0.0,
        }
    }
}

// Observer
pub fn obs_spitter_spit_action(
    trigger: Trigger<PartAction>,
    mut query: Query<(&Name, &mut CubeSpitter)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    transform_query: Query<&GlobalTransform>,
) {
    if let Ok((name, mut spitter)) = query.get_mut(trigger.target()) {
        // action identity check
        info!("spitter received action: {:?}", trigger.action_tag);
        if &trigger.action_tag == "spit" {
            info!("Spitting cube at {:?}", name);
            spitter.check_auto_spit(
                trigger.target(),
                &mut commands,
                &mut meshes,
                &mut materials,
                transform_query,
            );
        }
    }
}

fn spitter_cooldown_system(spitters: Query<&mut CubeSpitter>, delta_time: Res<Time>) {
    for mut spitter in spitters {
        spitter.tick_cooldown(delta_time.delta_secs());
    }
}

fn watchtower_request_spit_system(
    mut commands: Commands,
    watchtowers: Query<(Entity, &Name, &WatchtowerMarker)>,
) {
    for (entity, _name, _wm) in watchtowers {
        dispatch_action(
            &mut commands,
            entity,
            "spit".into(),
            vec!["spitter".into()],
            Box::from(()),
        );
    }
}

impl Default for CubeSpitter {
    fn default() -> Self {
        Self {
            interval: 2.0,
            vel: Vec3::ZERO,
            spawn_offset: Vec3::ZERO,
            cooldown: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
struct WatchtowerSpawnRequest {
    pub name: Option<String>,
    pub at: Vec3,
    pub thickness: f32,
    pub height: f32,
    pub num_spitters: u8,
    pub min_interval: f32,
    pub max_interval: f32,
}

// Cube spitter utility initializer.
fn spawn_cube_spitter_on_slot(
    name: String,
    slot: Entity,
    interval: f32,
    vel: Vec3,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Entity {
    let mesh = Mesh3d(meshes.add(Sphere::new(0.3)));

    let material = MeshMaterial3d(materials.add(StandardMaterial {
        base_color: Color::srgba_u8(255, 30, 0, 80),
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    }));

    let spitter_entity = commands
        .spawn((mesh, material, Name::new(name), part_tag("spitter".into())))
        .id();

    commands
        .entity(spitter_entity)
        .insert(CubeSpitter::new(interval, vel));

    install_part_on_slot(commands, spitter_entity, slot);

    spitter_entity
}

#[derive(Component)]
struct WatchtowerMarker;

/// Watchtower utility initializer.
fn spawn_watchtower(
    request: WatchtowerSpawnRequest,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let thickness = request.thickness;
    let height = request.height;
    let interval_distrib =
        Uniform::new_inclusive(request.min_interval, request.max_interval).unwrap();

    let name_string = if let Some(request_name) = request.name {
        format!("watchtower {}", request_name)
    } else {
        "watchtower".to_owned()
    };
    let name = Name::new(name_string.clone());

    // watchtower shape
    let mesh = Mesh3d(meshes.add(Cuboid::new(thickness, height, thickness)));

    // watchtower material
    let material = MeshMaterial3d(materials.add(StandardMaterial {
        base_color: Color::srgba_u8(190, 120, 10, 230),
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    }));

    // watchtower position
    let transform = Transform::from_translation(request.at + Vec3::Y * request.height / 2.0);

    // spawn watchtower
    let watchtower = commands
        .spawn((name, mesh, material, transform, WatchtowerMarker))
        .id();

    for i in 1..=request.num_spitters {
        // calculate offset
        let offset_vert = request.height * 0.8;
        let offset_horz_mag = request.thickness * 1.15;
        let offset_horz = Quat::from_rotation_y(
            std::f32::consts::PI * 2.0 * ((i - 1) as f32) / request.num_spitters as f32,
        ) * (Vec3::X * offset_horz_mag);
        let offset = offset_horz.with_y(offset_vert);

        let launch_vel = (offset_horz * 30.0).with_y(5.0);

        // add spitter slot
        let slot_entity = commands
            .spawn((
                Name::new(format!("slot {} of {}", i, name_string)),
                part_slot("spitter".into()),
                Transform::from_translation(offset),
            ))
            .id();

        commands
            .entity(watchtower)
            .add_one_related::<SlotOfConstruct>(slot_entity);

        // spawn spitter and add it to slot
        let this_interval = interval_distrib.sample(&mut rand::rng());
        spawn_cube_spitter_on_slot(
            format!("spitter {} of {}", i, name_string),
            slot_entity,
            this_interval,
            launch_vel,
            commands,
            meshes,
            materials,
        );
    }

    fn _watchtower_part_printer(
        In(watchtower): In<Entity>,
        part_query: Query<&ConstructParts>,
        partinfo_query: Query<&PartInfo>,
    ) {
        info!(
            "Infos on parts spawned on watchtower: {}",
            part_query
                .get(watchtower)
                .unwrap()
                .iter()
                .map(|&part_entity| -> String {
                    partinfo_query.get(part_entity).map_or_else(
                        |err| format!("({})", err),
                        |info| format!("[tags: {}]", info.tags.iter().join(",")),
                    )
                })
                .join(", ")
        );
    }

    commands.run_system_cached_with(_watchtower_part_printer, watchtower);
}

/// Point netowrk snapping market component.
#[derive(Component, Default)]
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

                    let front = network.points[0].pos;
                    let up = network.points[2].pos;
                    let up = (up - avg).normalize();

                    transform.translation = avg;
                    transform.look_at(front, up);
                }
            }
        },
    );

    app.add_systems(Startup, setup);

    app.add_systems(
        Update,
        (
            watchtower_request_spit_system,
            spitter_cooldown_system,
            lifetime_system,
        ),
    );

    app.add_observer(obs_spitter_spit_action);
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
    export_sources: Option<ResMut<Assets<ImageExportSource>>>,
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
            base_color: Color::srgba_u8(190, 190, 255, 90),
            alpha_mode: AlphaMode::Multiply,
            ..Default::default()
        })),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
            .with_translation(Vec3::new(0.0, -0.5, 0.0)),
    ));

    // light
    commands.spawn((
        PointLight {
            intensity: 20.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(15.0, 6.0, -8.0),
    ));

    // camera
    commands
        .spawn((
            Camera3d::default(),
            Transform::from_xyz(-5.0, 9.0, 18.0).looking_at(Vec3::Y * -0.5, Vec3::Y),
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

    // watchtower
    let request = WatchtowerSpawnRequest {
        name: None,
        at: Vec3::new(0.0, 0.0, 0.0),
        thickness: 2.0,
        height: 5.0,
        num_spitters: 3,
        min_interval: 0.8,
        max_interval: 1.5,
    };
    spawn_watchtower(request, &mut commands, &mut meshes, &mut materials);

    // start image exportation
    if let Some(mut export_sources) = export_sources {
        commands.spawn((
            ImageExport(export_sources.add(ImageExportSource(output_texture_handle.clone()))),
            ImageExportSettings {
                // Frames will be saved to "./out/soft-cube-buoyancy/[#####].png"
                // [NOTE] update output dir when grafting this code onto other examples
                output_dir: "out/soft-cube-buoyancy/".into(),

                // Choose "exr" for HDR renders (requires feature on crate bevy_image_export)
                extension: "png".into(),
            },
        ));
    }
}

/// Bundle for spawning a soft body cube.
#[derive(Bundle, Builder)]
struct CubeBundle<M: Material> {
    points: PointNetwork,
    springs: SpringNetwork,
    volumes: VolumeCollection,

    #[builder(default)]
    water_physics: WaterPhysics,

    #[builder(default)]
    air_drag: AirDrag,

    #[builder(default)]
    gravity: Gravity,

    #[builder(setter(skip), default)]
    snap_to_points: SnapToPointNet,

    mesh: Mesh3d,
    material: MeshMaterial3d<M>,

    transform: Transform,
}

impl<M: Material> CubeBundle<M> {
    pub fn builder() -> CubeBundleBuilder<M> {
        Default::default()
    }
}

// limited lifetime entities
#[derive(Component)]
struct Lifetime(pub f32);

fn lifetime_system(mut commands: Commands, query: Query<(Entity, &mut Lifetime)>, time: Res<Time>) {
    for (entity, mut lifetime) in query {
        lifetime.0 -= time.delta_secs();

        if lifetime.0 <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
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
        .map(|arr| PhysPoint::from_pos(at + Vec3::from(arr)))
        .into_iter(),
    );

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

    // generate point network visualization as little children balls
    let children = (0..points.points.len())
        .map(|point_idx| {
            let point_mesh = meshes.add(Sphere::new(0.05));
            let point_material = materials.add(StandardMaterial {
                base_color: Color::srgba_u8(255, 255, 48, 200),
                alpha_mode: AlphaMode::Blend,
                ..Default::default()
            });

            // child point
            commands
                .spawn((
                    PointAttach { point_idx },
                    Mesh3d(point_mesh),
                    MeshMaterial3d(point_material),
                    Transform::default(),
                ))
                .id()
        })
        .collect::<Vec<_>>();

    // create cube entity
    let cube = commands
        .spawn((
            CubeBundle::builder()
                .mesh(Mesh3d(cube_mesh))
                .material(MeshMaterial3d(cube_material))
                .transform(Transform::default())
                .points(points)
                .springs(springs)
                .volumes(volumes)
                .water_physics(WaterPhysics {
                    water_level: -1.5,

                    // exaggerated for demonstrative purposes
                    buoyancy_factor: 4.0,

                    ..Default::default()
                })
                .gravity(Gravity {
                    // low grav for development purposes
                    force: Vec3::Y * -3.0,
                })
                .build()
                .unwrap(),
            Lifetime(6.0),
        ))
        .id();

    commands.entity(cube).add_children(&children);

    cube
}

fn main() {
    let mut app = App::new();

    // image export

    // default plugin & main properties
    app.add_plugins((DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "Loot & Roam Tech Demo - Watchtower".into(),
                name: Some("bevy.loot-and-roam.techdemo.watchtower".into()),
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        })
        .set(RenderPlugin {
            synchronous_pipeline_compilation: true,
            ..default()
        }),));

    let export_plugin = if cfg!(not(debug_assertions)) {
        Some(ImageExportPlugin::default())
    } else {
        None
    };

    let export_threads = if let Some(export_plugin) = export_plugin {
        let threads = Some(export_plugin.threads.clone());
        app.add_plugins(export_plugin);

        threads
    } else {
        None
    };

    // engine systems
    app.add_plugins((
        CommonPlugin,
        AppPlugin,
        FrameTimeDiagnosticsPlugin::default(),
    ));

    // system registration
    apply_example_systems(&mut app);

    // logger
    app.add_plugins(LogDiagnosticsPlugin::default());

    app.run();

    // block till image sequence exportation is done
    if let Some(export_threads) = export_threads {
        export_threads.finish();
    }

    // command to render to video:
    // $ ffmpeg -r 60 -i out/soft-cube-buoyancy/%05d.png -vcodec libx264 -crf 25 -pix_fmt yuv420p out/soft-cube-buoyancy.mp4
    // command to reset demo recordings:
    // $ rm -r out/
}
