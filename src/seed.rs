use bevy::{input::common_conditions::input_just_pressed, prelude::*};

#[derive(Resource)]
pub struct Seed(pub u64);

pub struct SeedPlugin;

impl Plugin for SeedPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Seed(rand::random()))
            .add_systems(Update, reset_seed.run_if(input_just_pressed(KeyCode::KeyR)));
    }
}

fn reset_seed(mut seed: ResMut<Seed>) {
    seed.0 = rand::random();
}
