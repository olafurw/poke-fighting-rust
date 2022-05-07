use clap::Parser;
use nannou::prelude::*;

mod types;
use crate::types::*;

mod pokemon;
use crate::pokemon::*;

mod battle;
use crate::battle::*;

/// Pokemon battle simulation
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args
{
    /// Image width
    #[clap(short='w', long, default_value_t = 512)]
    width: usize,

    /// Image height
    #[clap(short='h', long, default_value_t = 512)]
    height: usize,

    /// When fighting, select random neighbour instead of the weakest one
    #[clap(short='r', long)]
    random: bool,
}

fn main()
{
    nannou::app(model).update(update).exit(exit).run();
}

struct Model
{
    battle: Battle,
    image: nannou::image::DynamicImage,
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
       .build()
       .unwrap();

    Model {
        battle: Battle::new(img_width, img_height, selection_algorithm),
        image: nannou::image::DynamicImage::ImageRgb8(nannou::image::RgbImage::new(img_width as u32, img_height as u32)),
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
            *pixel = nannou::image::Rgb(pokemon.kind.into());
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame)
{
    let texture = wgpu::Texture::from_image(app, &model.image);

    let draw = app.draw();
    draw.texture(&texture);
    draw.to_frame(app, &frame).unwrap();
}

fn exit(_app: &App, _model: Model)
{

}
