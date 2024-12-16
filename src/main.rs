// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments)]

mod func_gen;
mod render;
mod seed;
mod visibility;

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use func_gen::*;
use render::{generate_image, RenderPlugin};
use seed::{Seed, SeedPlugin};
use visibility::VisibilityPlugin;

const IMAGE_WIDTH: u32 = 800;
const IMAGE_HEIGHT: u32 = 800;

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
        //.insert_resource(Time::<Fixed>::from_hz(44100.0))
        .add_systems(Startup, setup)
        .add_plugins(RenderPlugin)
        .add_plugins(VisibilityPlugin)
        .add_plugins(SeedPlugin)
        //.add_systems(FixedUpdate, draw)
        .run();
}

/// Generate a black image with the given dimensions

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
