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
    nannou::app(model).update(update).run();
}

struct Model
{
    battle: Battle,
    image: nannou::image::DynamicImage,
}

fn model(app: &App) -> Model
{
    app.new_window().size(IMG_SIZE as u32, IMG_SIZE as u32).view(view).build().unwrap();

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
            *pixel = nannou::image::Rgb(pokemon.kind.into());
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame)
{
    let texture = wgpu::Texture::from_image(app, &model.image);

    frame.clear(PURPLE);

    let draw = app.draw();
    draw.texture(&texture);
    draw.to_frame(app, &frame).unwrap();
}
