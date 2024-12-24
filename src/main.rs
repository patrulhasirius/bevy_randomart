// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments)]

mod func_gen;
mod gpu_draw;
mod render;
mod seed;
mod state;
mod visibility;

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use func_gen::*;
use gpu_draw::GpuRenderPlugin;
use render::{generate_image, CpuRenderPlugin};
use seed::{Seed, SeedPlugin};
use state::StatePlugin;
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
        .add_plugins(CpuRenderPlugin)
        .add_plugins(VisibilityPlugin)
        .add_plugins(SeedPlugin)
        .add_plugins(GpuRenderPlugin)
        .add_plugins(StatePlugin)
        .run();
}

/// Generate a black image with the given dimensions
fn setup(mut commands: Commands, seed: Res<Seed>) {
    info!("seed: {}", seed.0);
    // spawn a camera
    commands.spawn(Camera2d);
}
