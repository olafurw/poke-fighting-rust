use clap::Parser;
use nannou::prelude::*;
use nannou::image::GenericImageView;

mod types;
mod pokemon;
mod battle;
use battle::{Battle, SelectionAlgorithm};

/// Pokemon battle simulation
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args
{
    /// Image width
    #[clap(short='w', long, default_value_t = 512, validator = validate_size)]
    width: usize,

    /// Image height
    #[clap(short='h', long, default_value_t = 512, validator = validate_size)]
    height: usize,

    /// When fighting, select random neighbour instead of the weakest one
    #[clap(short='r', long)]
    random: bool,
}

fn main()
{
    nannou::app(model).update(update).exit(exit).run();
}

fn validate_size(arg: &str) -> Result<(), String>
{
    if let Ok(size) = arg.parse::<usize>()
    {
        // wgpu won't allow more than 8192 pixels
        if !(32..8193).contains(&size)
        {
            return Err("image size should be between 32 and 8192".to_string());
        }
    }

    Ok(())
}

struct Model
{
    battle: Battle,
    image: nannou::image::DynamicImage,
    window_width: u32,
    window_height: u32,
}

fn model(app: &App) -> Model
{
    let args = Args::parse();
    let img_width = args.width;
    let img_height = args.height;
    let selection_algorithm = if args.random { SelectionAlgorithm::RandomNeighbour } else { SelectionAlgorithm::WeakestNeighbour };

    let surface_conf_builder = nannou::window::SurfaceConfigurationBuilder::new().present_mode(nannou::wgpu::PresentMode::Mailbox);
    app.new_window()
       .size(img_width as u32, img_height as u32)
       .surface_conf_builder(surface_conf_builder)
       .clear_color(PURPLE)
       .view(view)
       .event(event)
       .build()
       .unwrap();

    Model {
        battle: Battle::new(img_width, img_height, selection_algorithm),
        image: nannou::image::DynamicImage::ImageRgb8(nannou::image::RgbImage::new(img_width as u32, img_height as u32)),
        window_width: img_width as u32,
        window_height: img_height as u32,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update)
{
    model.battle.action();

    if let nannou::image::DynamicImage::ImageRgb8(ref mut pixels) = model.image
    {
        for (x, y, pixel) in pixels.enumerate_pixels_mut()
        {
            let pokemon = model.battle.pokemon(x, y);
            *pixel = pokemon.kind.into();
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame)
{
    let texture = wgpu::Texture::from_image(app, &model.image);
    let (image_width, image_height) = model.image.dimensions();
    let width_ratio = model.window_width as f32 / image_width as f32;
    let height_ratio = model.window_height as f32 / image_height as f32;
    let ratio = if width_ratio < height_ratio { width_ratio } else { height_ratio };

    let draw = app.draw();
    if ratio == 1.0
    {
        draw.texture(&texture);
    }
    else
    {
        draw.texture(&texture).width(ratio * image_width as f32).height(ratio * image_height as f32);
    }
    draw.to_frame(app, &frame).unwrap();
}

fn event(_app: &App, model: &mut Model, ev: nannou::event::WindowEvent)
{
    if let WindowEvent::Resized(resized) = &ev
    {
        model.window_width = resized.x as u32;
        model.window_height = resized.y as u32;
    }
}


fn exit(_app: &App, _model: Model)
{

}
