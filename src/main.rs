use nannou::prelude::*;

mod types;
use crate::types::*;

mod pokemon;
use crate::pokemon::*;

mod battle;
use crate::battle::*;

pub const IMG_SIZE: usize = 512;

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
    let surface_conf_builder = nannou::window::SurfaceConfigurationBuilder::new().present_mode(nannou::wgpu::PresentMode::Mailbox);
    app.new_window()
       .size(IMG_SIZE as u32, IMG_SIZE as u32)
       .surface_conf_builder(surface_conf_builder)
       .clear_color(PURPLE)
       .view(view)
       .build()
       .unwrap();

    Model {
        battle: Battle::new(),
        image: nannou::image::DynamicImage::ImageRgb8(nannou::image::RgbImage::new(IMG_SIZE as u32, IMG_SIZE as u32)),
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

    let draw = app.draw();
    draw.texture(&texture);
    draw.to_frame(app, &frame).unwrap();
}

fn exit(_app: &App, _model: Model)
{

}
