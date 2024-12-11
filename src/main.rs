// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::asset::AssetMetaCheck;
use bevy::color::palettes::css;
use bevy::math::U64Vec2;
use bevy::prelude::*;
use bevy::render::{
    render_asset::RenderAssetUsages,
    render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy::window::WindowResized;
use rand::Rng;

const NANOS_MULTIPLIER: u64 = 10;
const IMAGE_WIDTH: u32 = 800;
const IMAGE_HEIGHT: u32 = 600;

#[derive(Resource)]
struct PixelTracker {
    counter: u64,
}

impl PixelTracker {
    pub const ZERO: Self = Self { counter: 0 };
}

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
        //.insert_resource(Time::<Fixed>::from_hz(44100.0))
        .insert_resource(PixelTracker::ZERO)
        .add_systems(Startup, setup)
        .add_systems(Update, (on_resize_system, draw))
        //.add_systems(FixedUpdate, draw)
        .run();
}

/// Store the image handle that we will draw to, here.
#[derive(Resource)]
struct MyProcGenImage(Handle<Image>);

fn generate_image(width: u32, height: u32) -> Image {
    // create an image that we are going to draw into
    Image::new_fill(
        // 2D image of size 256x256
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        // Initialize it with a beige color
        &(css::BLACK.to_u8_array()),
        // Use the same encoding as the color we set
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
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
    mut pixel_tracker: ResMut<PixelTracker>,
    time: Res<Time>,
) {
    let i = pixel_tracker.counter;
    let mut rng = rand::thread_rng();

    // Get the image from Bevy's asset storage.
    let image = images.get_mut(&my_handle.0).expect("Image not found");

    let nanos = time.delta().as_millis();

    for n in 0..(nanos as u64) * NANOS_MULTIPLIER {
        if (i + n) <= (image.height() * image.width()).into() {
            // Generate a random color.
            let draw_color = Color::linear_rgba(rng.gen(), rng.gen(), rng.gen(), rng.gen());

            let xy = U64Vec2::new(
                (i + n) % image.width() as u64,
                (i + n) / image.width() as u64,
            );
            let (x, y) = (xy.x, xy.y);

            // Set the new color, but keep old alpha value from image.
            image
                .set_color_at(
                    x as u32, y as u32, draw_color, //.with_alpha(rng.gen_range(0.0..1.0)),
                ) //.with_alpha(old_color.alpha()))
                .unwrap();

            //info!("pixel count {}", pixel_tracker.counter);
        }
    }
    pixel_tracker.counter += (nanos as u64) * NANOS_MULTIPLIER;
}

fn on_resize_system(
    mut commands: Commands,
    mut resize_reader: EventReader<WindowResized>,
    query: Query<Entity, With<Sprite>>,
    mut images: ResMut<Assets<Image>>,
    mut pixel_tracker: ResMut<PixelTracker>,
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

        *pixel_tracker = PixelTracker::ZERO;
    }
}
