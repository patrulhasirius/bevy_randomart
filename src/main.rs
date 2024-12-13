// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod func_gen;

use bevy::asset::AssetMetaCheck;
use bevy::color::palettes::css;
use bevy::core::FrameCount;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy::render::{
    render_asset::RenderAssetUsages,
    render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy::window::{WindowResized, WindowResolution};
use func_gen::*;
use rand::SeedableRng;

const IMAGE_WIDTH: u32 = 800;
const IMAGE_HEIGHT: u32 = 800;

#[derive(Resource)]
struct Seed(u64);

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
                        resolution: WindowResolution::new(IMAGE_HEIGHT as f32, IMAGE_WIDTH as f32),
                        canvas: Some("#canvas".into()),
                        fit_canvas_to_parent: true,
                        //prevent_default_event_handling: false,
                        visible: false,

                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(Seed(rand::random()))
        //.insert_resource(Time::<Fixed>::from_hz(44100.0))
        .add_systems(Startup, setup)
        .add_systems(Update, on_resize_system.run_if(window_resized))
        .add_systems(
            Update,
            (reset_seed, on_seed_reset)
                .chain()
                .run_if(input_just_pressed(KeyCode::KeyR)),
        )
        .add_systems(Update, make_visible)
        //.add_systems(FixedUpdate, draw)
        .run();
}

/// Generate a black image with the given dimensions
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

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>, seed: Res<Seed>) {
    info!("seed: {}", seed.0);
    // spawn a camera
    commands.spawn(Camera2d);

    let image = generate_image(IMAGE_WIDTH, IMAGE_HEIGHT);

    // add it to Bevy's assets, so it can be used for rendering
    // this will give us a handle we can use
    // (to display it in a sprite, or as part of UI, etc.)

    let handle = images.add(image);

    // create a sprite entity using our image
    commands.spawn(Sprite::from_image(handle.clone()));
}

fn render_pixels(image: &mut Image, seed: u64) {
    const MAX_DEPTH: u32 = 50;
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

    let r_tree = generate_tree(MAX_DEPTH, &mut rng);
    let g_tree = generate_tree(MAX_DEPTH, &mut rng);
    let b_tree = generate_tree(MAX_DEPTH, &mut rng);

    let mut buffer: Vec<u8> = Vec::with_capacity((image.height() * image.height()) as usize);
    for y in 0..image.height() {
        // 0..height => 0..2 => -1..1
        let ny = (y as f32) / ((image.height() - 1) as f32) * 2. - 1.;
        for x in 0..image.width() {
            let nx = (x as f32) / ((image.width() - 1) as f32) * 2. - 1.;
            let result_r = eval(nx, ny, &r_tree, &mut rng);
            let result_g = eval(nx, ny, &g_tree, &mut rng);
            let result_b = eval(nx, ny, &b_tree, &mut rng);

            buffer.push((result_r / 2. * 255.) as u8);
            buffer.push((result_g / 2. * 255.) as u8);
            buffer.push((result_b / 2. * 255.) as u8);
            buffer.push(255);

            //let color = Color::linear_rgb(result_r, result_g, result_b);
            //image.set_color_at(x, y, color).unwrap();
        }
    }
    //println!("{:#?}", buffer);
    *image = Image::new(
        Extent3d {
            width: image.width(),
            height: image.height(),
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        buffer,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
}

/// On screen resize respawn the sprite, redraw the image and reset pixel counter
fn on_resize_system(
    mut commands: Commands,
    query: Query<Entity, With<Sprite>>,
    mut images: ResMut<Assets<Image>>,
    seed: Res<Seed>,
    windows: Query<&Window>,
) {
    let Ok(sprite) = query.get_single() else {
        return;
    };

    let window = windows.single();
    // When resolution is being changed
    commands.entity(sprite).despawn_recursive();
    let id = images.ids().last().unwrap();
    let _ = images.remove_untracked(id).unwrap();

    let mut image = generate_image(
        window.resolution.width() as u32,
        window.resolution.height() as u32,
    );

    render_pixels(&mut image, seed.0);

    let handle = images.add(image);
    commands.spawn(Sprite::from_image(handle.clone()));
}

fn on_seed_reset(
    mut commands: Commands,
    query: Query<Entity, With<Sprite>>,
    mut images: ResMut<Assets<Image>>,
    seed: Res<Seed>,
    windows: Query<&Window>,
) {
    info!("seed: {}", seed.0);
    let Ok(sprite) = query.get_single() else {
        return;
    };
    let window = windows.single();
    let id = images.ids().last().unwrap();
    let _ = images.remove_untracked(id).unwrap();
    let mut image = generate_image(
        window.resolution.width() as u32,
        window.resolution.height() as u32,
    );
    render_pixels(&mut image, seed.0);

    commands.entity(sprite).despawn_recursive();

    let handle = images.add(image);
    commands.spawn(Sprite::from_image(handle.clone()));
}

fn reset_seed(mut seed: ResMut<Seed>) {
    seed.0 = rand::random();
}

fn window_resized(mut resize_reader: EventReader<WindowResized>) -> bool {
    resize_reader.read().last().is_some()
}

fn make_visible(mut window: Single<&mut Window>, frames: Res<FrameCount>) {
    if frames.0 == 10 {
        window.visible = true;
    }
}
