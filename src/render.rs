use bevy::{
    asset::RenderAssetUsages,
    color::palettes::css,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    tasks::{ComputeTaskPool, ParallelSlice},
    window::WindowResized,
};
use rand::SeedableRng;
use rayon::prelude::*;

use crate::{eval, generate_tree, seed::Seed};

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (render).run_if(should_run));
    }
}

fn should_run(mut resize_reader: EventReader<WindowResized>, seed: Res<Seed>) -> bool {
    resize_reader.read().last().is_some() | seed.is_changed()
}

fn render(
    mut query: Query<&mut Sprite>,
    mut images: ResMut<Assets<Image>>,
    seed: Res<Seed>,
    windows: Query<&Window>,
) {
    let window = windows.single();

    if let Ok(mut sprite) = query.get_single_mut() {
        // When resolution is being changed
        let mut image = generate_image(
            window.resolution.width() as u32,
            window.resolution.height() as u32,
        );

        render_pixels(&mut image, seed.0);

        let handle = images.add(image);
        sprite.image = handle.clone()
    };
}

pub fn generate_image(width: u32, height: u32) -> Image {
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

    //let zipped: Vec<(f32, f32, f32)> = (0..height)
    //    .into_par_iter()
    //    .flat_map(|y| {
    //        let ny = (y as f32) / (height as f32) * 2. - 1.;
    //        (0..width)
    //            .map(|x| {
    //                let nx = (x as f32) / (width as f32) * 2. - 1.;

    //                let result_r = eval(nx, ny, &r_tree);
    //                let result_g = eval(nx, ny, &g_tree);
    //                let result_b = eval(nx, ny, &b_tree);

    //                (result_r, result_g, result_b)
    //            })
    //            .collect::<Vec<(f32, f32, f32)>>()
    //    })
    //    .collect();

    let result: Vec<u8> = (0..height)
        .collect::<Vec<u32>>()
        .par_splat_map(ComputeTaskPool::get(), None, |_, data| {
            let mut vec: Vec<f32> = Vec::new();
            data.iter().for_each(|y| {
                let ny = (*y as f32) / (height as f32) * 2. - 1.;
                (0..width).for_each(|x| {
                    let nx = (x as f32) / (width as f32) * 2. - 1.;

                    vec.push(eval(nx, ny, &r_tree));
                    vec.push(eval(nx, ny, &g_tree));
                    vec.push(eval(nx, ny, &b_tree));
                    vec.push(1.);
                })
            });
            vec
        })
        .into_iter()
        .flatten()
        .collect::<Vec<f32>>()
        .iter()
        .map(|n| ((n + 1.) / 2. * 255.) as u8)
        .collect();

    //let (buffer_r, buffer_g, buffer_b): (Vec<_>, Vec<_>, Vec<_>) = itertools::multiunzip(zipped);

    // let buffer_r_max = buffer_r.iter().max_by(|a, b| a.total_cmp(b)).unwrap();
    // let buffer_r_min = buffer_r.iter().min_by(|a, b| a.total_cmp(b)).unwrap();

    //let buffer_r: Vec<u8> = buffer_r
    //    .iter()
    //    // this method sounds more correct but looks way worse
    //    //.map(|n| ((n + buffer_r_min.abs()) / (buffer_r_max + buffer_r_min.abs()) * 255.) as u8)
    //    .map(|n| ((n + 1.) / 2. * 255.) as u8)
    //    .collect();

    // let buffer_g_max = buffer_g.iter().max_by(|a, b| a.total_cmp(b)).unwrap();
    // let buffer_g_min = buffer_g.iter().min_by(|a, b| a.total_cmp(b)).unwrap();

    // let buffer_g: Vec<u8> = buffer_g
    //     .iter()
    //     //.map(|x| ((x + buffer_g_min.abs()) / (buffer_g_max + buffer_g_min.abs()) * 255.) as u8)
    //     .map(|n| ((n + 1.) / 2. * 255.) as u8)
    //     .collect();

    // let buffer_b_max = buffer_b.iter().max_by(|a, b| a.total_cmp(b)).unwrap();
    // let buffer_b_min = buffer_b.iter().min_by(|a, b| a.total_cmp(b)).unwrap();

    // let buffer_b: Vec<u8> = buffer_b
    //     .iter()
    //     //.map(|x| ((x + buffer_b_min.abs()) / (buffer_b_max + buffer_b_min.abs()) * 255.) as u8)
    //     .map(|n| ((n + 1.) / 2. * 255.) as u8)
    //     .collect();

    // let buffer_a: Vec<u8> = vec![255; (image.height() * image.width()) as usize];

    // let interleaved: Vec<u8> = buffer_r
    //     .iter()
    //     .zip(buffer_g.iter())
    //     .zip(buffer_b.iter())
    //     .zip(buffer_a.iter())
    //     .flat_map(|(((a, b), c), d)| [a, b, c, d])
    //     .cloned()
    //     .collect();

    //info!("{:#?}", interleaved);
    *image = Image::new(
        Extent3d {
            width: image.width(),
            height: image.height(),
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        result,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
}
