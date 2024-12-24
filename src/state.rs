use bevy::{ecs::system::QueryLens, input::common_conditions::input_just_pressed, prelude::*};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum RenderState {
    #[default]
    GpuRender,
    CpuRender,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<RenderState>().add_systems(
            Update,
            toggle_cpu_render.run_if(input_just_pressed(KeyCode::KeyC)),
        );
    }
}

// Change render method and despawn everything
fn toggle_cpu_render(
    mut commands: Commands,
    sprites: Query<Entity, With<Sprite>>,
    meshes: Query<Entity, With<Mesh2d>>,
    mut next_state: ResMut<NextState<RenderState>>,
    state: Res<State<RenderState>>,
) {
    let next = match state.get() {
        RenderState::GpuRender => RenderState::CpuRender,
        RenderState::CpuRender => RenderState::GpuRender,
    };
    next_state.set(next);

    for entity in sprites.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for entity in meshes.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
