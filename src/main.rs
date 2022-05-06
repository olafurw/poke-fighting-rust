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
}

fn model(app: &App) -> Model
{
    app.new_window().size(IMG_SIZE as u32, IMG_SIZE as u32).view(view).build().unwrap();

    Model {
        battle: Battle::new()
    }
}

fn update(_app: &App, model: &mut Model, _update: Update)
{
    model.battle.action();
}

fn view(app: &App, model: &Model, frame: Frame)
{
    let mut pixels = nannou::image::RgbImage::new(IMG_SIZE as u32, IMG_SIZE as u32);

    for y in 0..IMG_SIZE
    {
        for x in 0..IMG_SIZE
        {
            let pokemon = &model.battle.pokemons[y][x];
            let color: [u8; 3] = pokemon.kind.into();
            pixels.put_pixel(x as u32, y as u32, nannou::image::Rgb(color));
        }
    }

    let image = nannou::image::DynamicImage::ImageRgb8(pixels);
    let texture = wgpu::Texture::from_image(app, &image);

    frame.clear(PURPLE);

    let draw = app.draw();
    draw.texture(&texture);
    draw.to_frame(app, &frame).unwrap();
}
