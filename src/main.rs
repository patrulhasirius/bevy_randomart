// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::asset::AssetMetaCheck;
use bevy::color::{color_difference::EuclideanDistance, palettes::css};
use bevy::prelude::*;
use bevy::render::{
    render_asset::RenderAssetUsages,
    render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy::window::WindowResized;
use rand::Rng;

const IMAGE_WIDTH: u32 = 800;
const IMAGE_HEIGHT: u32 = 600;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics in web builds on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(Time::<Fixed>::from_hz(10024.0))
        .add_systems(Startup, setup)
        .add_systems(Update, on_resize_system)
        .add_systems(FixedUpdate, draw)
        .run();
}

/// Store the image handle that we will draw to, here.
#[derive(Resource)]
struct MyProcGenImage(Handle<Image>);

fn generate_image(width: u32, height: u32) -> Image {
    // create an image that we are going to draw into
    let mut image = Image::new_fill(
        // 2D image of size 256x256
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        // Initialize it with a beige color
        &(css::BEIGE.to_u8_array()),
        // Use the same encoding as the color we set
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    // to make it extra fancy, we can set the Alpha of each pixel
    // so that it fades out in a circular fashion
    for y in 0..image.height() {
        for x in 0..image.width() {
            let center = Vec2::new(image.width() as f32 / 2.0, image.height() as f32 / 2.0);
            let max_radius = image.height().min(image.width()) as f32 / 2.0;
            let r = Vec2::new(x as f32, y as f32).distance(center);
            let a = 1.0 - (r / max_radius).clamp(0.0, 1.0);

            // here we will set the A value by accessing the raw data bytes
            // (it is the 4th byte of each pixel, as per our `TextureFormat`)

            // find our pixel by its coordinates
            let pixel_bytes = image.pixel_bytes_mut(UVec3::new(x, y, 0)).unwrap();
            // convert our f32 to u8
            pixel_bytes[3] = (a * u8::MAX as f32) as u8;
        }
    }

    image
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    // spawn a camera
    commands.spawn(Camera2d);

    let image = generate_image(IMAGE_WIDTH, IMAGE_HEIGHT);

    // add it to Bevy's assets, so it can be used for rendering
    // this will give us a handle we can use
    // (to display it in a sprite, or as part of UI, etc.)
    let handle = images.add(image);

    // create a sprite entity using our image
    commands.spawn(Sprite::from_image(handle.clone()));
    commands.insert_resource(MyProcGenImage(handle));
}

/// Every fixed update tick, draw one more pixel to make a spiral pattern
fn draw(
    my_handle: Res<MyProcGenImage>,
    mut images: ResMut<Assets<Image>>,
    // used to keep track of where we are
    mut i: Local<u32>,
    mut draw_color: Local<Color>,
) {
    let mut rng = rand::thread_rng();

    if *i == 0 {
        // Generate a random color on first run.
        *draw_color = Color::linear_rgb(rng.gen(), rng.gen(), rng.gen());
    }

    // Get the image from Bevy's asset storage.
    let image = images.get_mut(&my_handle.0).expect("Image not found");

    // Compute the position of the pixel to draw.

    let center = Vec2::new(image.width() as f32 / 2.0, image.height() as f32 / 2.0);
    let max_radius = image.height().min(image.width()) as f32 / 2.0;
    let rot_speed = 0.0123;
    let period = 0.12345;

    let r = ops::sin(*i as f32 * period) * max_radius;
    let xy = Vec2::from_angle(*i as f32 * rot_speed) * r + center;
    let (x, y) = (xy.x as u32, xy.y as u32);

    // Get the old color of that pixel.
    let old_color = image.get_color_at(x, y).unwrap();

    // If the old color is our current color, change our drawing color.
    let tolerance = 1.0 / 255.0;
    if old_color.distance(&draw_color) <= tolerance {
        *draw_color = Color::linear_rgb(rng.gen(), rng.gen(), rng.gen());
    }

    // Set the new color, but keep old alpha value from image.
    image
        .set_color_at(x, y, draw_color.with_alpha(old_color.alpha()))
        .unwrap();

    *i += 1;
}

fn on_resize_system(
    mut commands: Commands,
    mut resize_reader: EventReader<WindowResized>,
    query: Query<Entity, With<Sprite>>,
    mut images: ResMut<Assets<Image>>,
) {
    let Ok(sprite) = query.get_single() else {
        return;
    };
    if let Some(e) = resize_reader.read().last() {
        // When resolution is being changed
        commands.entity(sprite).despawn_recursive();
        commands.remove_resource::<MyProcGenImage>();

        let image = generate_image(e.width as u32, e.height as u32);

        let handle = images.add(image);
        commands.spawn(Sprite::from_image(handle.clone()));
        commands.insert_resource(MyProcGenImage(handle));
    }
}
