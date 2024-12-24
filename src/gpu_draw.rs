use bevy::{
    color::palettes::css::RED,
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin},
    window::WindowResized,
};
use rand::SeedableRng;

use crate::{generate_tree, render::generate_image, seed::Seed, state::RenderState};

pub const MESH2D_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(6942000000000);

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    color: LinearRgba,
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        MESH2D_SHADER_HANDLE.into()
    }
}

pub struct GpuRenderPlugin;

impl Plugin for GpuRenderPlugin {
    fn build(&self, app: &mut App) {
        let mut shaders = app.world_mut().resource_mut::<Assets<Shader>>();
        shaders.insert(
            &MESH2D_SHADER_HANDLE,
            Shader::from_wgsl(
                r#"
            #import bevy_sprite::{
                mesh2d_vertex_output::VertexOutput,
                mesh2d_view_bindings::globals,
                }

            @group(2) @binding(0) var base_color_texture: texture_2d<f32>;

            @fragment
            fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
                return vec4f((sin(f32(-0.04351914))) % (f32((f32(-0.74157834)) > (f32(-0.02816999)))), sin(globals.time), sin(globals.time), 1.0);
            }
            "#,
                file!(),
            ),
        );

        app.add_plugins(Material2dPlugin::<CustomMaterial>::default())
            .add_systems(Update, gpu_draw.run_if(should_run));
    }
}

fn should_run(
    mut resize_reader: EventReader<WindowResized>,
    seed: Res<Seed>,
    state: Res<State<RenderState>>,
) -> bool {
    (resize_reader.read().last().is_some() | seed.is_changed() | state.is_changed())
        & (*state.get() == RenderState::GpuRender)
}

fn gpu_draw(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    mesh_entities: Query<Entity, With<Mesh2d>>,
    windows: Query<&Window>,
    mut shaders: ResMut<Assets<Shader>>,
    seed: ResMut<Seed>,
) {
    let window = windows.single();

    const MAX_DEPTH: u32 = 30;
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed.0);

    info!("{}", seed.0);

    let r_tree = generate_tree(MAX_DEPTH, &mut rng);
    // info!("{:?}", r_tree);
    let g_tree = generate_tree(MAX_DEPTH, &mut rng);
    // info!("{:?}", g_tree);
    let b_tree = generate_tree(MAX_DEPTH, &mut rng);
    // info!("{:?}", b_tree);

    shaders.insert(
        &MESH2D_SHADER_HANDLE,
        Shader::from_wgsl(
            format!(
                "
        #import bevy_sprite::{{
            mesh2d_vertex_output::VertexOutput,
            mesh2d_view_bindings::globals,
            }}

        @group(2) @binding(0) var base_color_texture: texture_2d<f32>;

        @fragment
        fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {{
            return vec4f(({}), ({}), ({}), (1.0));
        }}
        ",
                r_tree, g_tree, b_tree
            ),
            file!(),
        ),
    );

    mesh_entities
        .iter()
        .for_each(|entity| commands.entity(entity).despawn_recursive());

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(
            window.resolution.width(),
            window.resolution.height(),
        ))),
        MeshMaterial2d(materials.add(CustomMaterial {
            color: LinearRgba::BLUE,
        })),
    ));
}
