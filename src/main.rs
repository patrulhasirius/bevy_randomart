// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments)]

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
use rayon::prelude::*;

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
        .add_systems(Startup, (setup, on_seed_reset).chain())
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
        //TextureFormat::Rgba8Unorm,
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
    const MAX_DEPTH: u32 = 10;
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

    let r_tree = generate_tree(MAX_DEPTH, &mut rng);
    info!("{:?}", r_tree);
    let g_tree = generate_tree(MAX_DEPTH, &mut rng);
    info!("{:?}", g_tree);
    let b_tree = generate_tree(MAX_DEPTH, &mut rng);
    info!("{:?}", b_tree);

    info!("Generated");

    //let mut buffer_r: Vec<f32> = Vec::with_capacity((image.height() * image.height()) as usize);
    //let mut buffer_g: Vec<f32> = Vec::with_capacity((image.height() * image.height()) as usize);
    //let mut buffer_b: Vec<f32> = Vec::with_capacity((image.height() * image.height()) as usize);

    let (width, height) = (image.width(), image.height());

    let zipped: Vec<(f32, f32, f32)> = (0..height)
        .into_par_iter()
        .flat_map(|y| {
            let ny = (y as f32) / (height as f32) * 2. - 1.;
            (0..width)
                .map(|x| {
                    let nx = (x as f32) / (width as f32) * 2. - 1.;

                    let result_r = eval(nx, ny, &r_tree);
                    let result_g = eval(nx, ny, &g_tree);
                    let result_b = eval(nx, ny, &b_tree);

                    (result_r, result_g, result_b)
                })
                .collect::<Vec<(f32, f32, f32)>>()
        })
        .collect();

    let (buffer_r, buffer_g, buffer_b): (Vec<_>, Vec<_>, Vec<_>) = itertools::multiunzip(zipped);

    // let buffer_r_max = buffer_r.iter().max_by(|a, b| a.total_cmp(b)).unwrap();
    // let buffer_r_min = buffer_r.iter().min_by(|a, b| a.total_cmp(b)).unwrap();

    let buffer_r: Vec<u8> = buffer_r
        .iter()
        // this method sounds more correct but looks way worse
        //.map(|n| ((n + buffer_r_min.abs()) / (buffer_r_max + buffer_r_min.abs()) * 255.) as u8)
        .map(|n| ((n + 1.) / 2. * 255.) as u8)
        .collect();

    // let buffer_g_max = buffer_g.iter().max_by(|a, b| a.total_cmp(b)).unwrap();
    // let buffer_g_min = buffer_g.iter().min_by(|a, b| a.total_cmp(b)).unwrap();

    let buffer_g: Vec<u8> = buffer_g
        .iter()
        //.map(|x| ((x + buffer_g_min.abs()) / (buffer_g_max + buffer_g_min.abs()) * 255.) as u8)
        .map(|n| ((n + 1.) / 2. * 255.) as u8)
        .collect();

    // let buffer_b_max = buffer_b.iter().max_by(|a, b| a.total_cmp(b)).unwrap();
    // let buffer_b_min = buffer_b.iter().min_by(|a, b| a.total_cmp(b)).unwrap();

    let buffer_b: Vec<u8> = buffer_b
        .iter()
        //.map(|x| ((x + buffer_b_min.abs()) / (buffer_b_max + buffer_b_min.abs()) * 255.) as u8)
        .map(|n| ((n + 1.) / 2. * 255.) as u8)
        .collect();

    let buffer_a: Vec<u8> = vec![255; (image.height() * image.width()) as usize];

    let interleaved: Vec<u8> = buffer_r
        .iter()
        .zip(buffer_g.iter())
        .zip(buffer_b.iter())
        .zip(buffer_a.iter())
        .flat_map(|(((a, b), c), d)| [a, b, c, d])
        .cloned()
        .collect();

    //info!("{:#?}", interleaved);
    *image = Image::new(
        Extent3d {
            width: image.width(),
            height: image.height(),
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        interleaved,
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
