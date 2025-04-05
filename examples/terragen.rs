//! Perlin noise based terrain heightmap generation test demo.

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

use bevy::{
    asset::RenderAssetUsages,
    color::palettes::css,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use loot_and_roam::common::terrain::prelude::*;

/// Size of both sides of the image texture to be drawn onto.
const IMAGE_SIZE: u32 = 600;

//----

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    // spawn a camera
    commands.spawn(Camera2d);

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
            min_shore_distance: 30.0,
            max_shore_distance: 150.0,
            ..Default::default()
        })
        .center_points(vec![
            CenterPoint::new(Vec2::new(400.0, 400.0), 1.5),
            CenterPoint::new(Vec2::new(500.0, 50.0), 0.3),
        ])
        .build()
        .unwrap();

    // create an image to draw Perlin noise into
    let mut image = Image::new_fill(
        // 2D image of size 256x256
        Extent3d {
            width: IMAGE_SIZE,
            height: IMAGE_SIZE,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &(css::BLACK.to_u8_array()),
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    // draw noise (f32)
    for y in 0..IMAGE_SIZE {
        for x in 0..IMAGE_SIZE {
            // find bytes of this pixel by coordinates
            let pixel_bytes = image.pixel_bytes_mut(UVec3::new(x, y, 0)).unwrap();

            // map image coordinates to terrain coordinates
            let terra_x = x as f32 * 800.0 / IMAGE_SIZE as f32;
            let terra_y = y as f32 * 800.0 / IMAGE_SIZE as f32;

            // get noise value at coordinates
            let height = terragen.get_height_at(Vec2::new(terra_x, terra_y));

            // draw influence value
            // (red for positive, blue for negative)
            if height > 0.0 {
                pixel_bytes[2] = (height * 256.0).floor() as u8;
                pixel_bytes[1] = 255;
                pixel_bytes[0] = 127;
            } else {
                pixel_bytes[0] = (255 - (-height * 256.0).floor() as u8) / 2;
                pixel_bytes[1] = 10;
                pixel_bytes[2] = 200;
            }
        }
    }

    // spawn textured sprite
    let image_handle = images.add(image);
    commands.spawn(Sprite::from_image(image_handle.clone()));
}
