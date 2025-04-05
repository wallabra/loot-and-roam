//! Fractal Perlin noise test demo.

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
use loot_and_roam::common::terrain::noise::FractalNoise;

/// Size of both sides of the image texture to be drawn onto.
const IMAGE_SIZE: u32 = 600;

/// Number of quads on any given side of the noise texture.
/// Edit this constant to change the 'resolution' of the noise texture.
const QUAD_EXTENT: u32 = 5;

/// Size of each quad in pixels.
const QUAD_SIZE: f32 = IMAGE_SIZE as f32 / QUAD_EXTENT as f32;

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

    // initialize fractal Perlin noise
    let mut noise = FractalNoise::new(QUAD_EXTENT as f32 + 1.0, QUAD_EXTENT as f32 + 1.0);
    let mut rng = rand::rng();
    noise.add_many_random_octaves(5, &mut rng);

    let noise = noise; // make immutable

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

            // find normalized coordinates (quad size 1.0)
            let norm_x = (x as f32) / QUAD_SIZE;
            let norm_y = (y as f32) / QUAD_SIZE;

            // draw tile boundaries for debug
            if norm_x.fract() < 0.01
                || norm_y.fract() < 0.01
                || norm_x.fract() > 0.99
                || norm_y.fract() > 0.99
            {
                pixel_bytes[1] = 127;
            }

            // get noise map's influence value at coordinates
            let influence = noise.get_influence_at(norm_x, norm_y);

            // draw influence value
            // (red for positive, blue for negative)
            if influence > 0.0 {
                pixel_bytes[0] = (influence * 256.0).floor() as u8;
            } else {
                pixel_bytes[2] = (-influence * 256.0).floor() as u8;
            }
        }
    }

    // spawn textured sprite
    let image_handle = images.add(image);
    commands.spawn(Sprite::from_image(image_handle.clone()));
}
