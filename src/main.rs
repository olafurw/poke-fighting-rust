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
    app.new_window().size(IMG_SIZE as u32, IMG_SIZE as u32).event(event).view(view).build().unwrap();

    Model { 
        battle: Battle::new() 
    }
}

fn event(_app: &App, _model: &mut Model, _event: WindowEvent)
{
}

fn update(_app: &App, _model: &mut Model, _update: Update)
{
    _model.battle.action();
}

fn view(_app: &App, _model: &Model, frame: Frame)
{
    let mut pixels = nannou::image::RgbaImage::new(IMG_SIZE as u32, IMG_SIZE as u32);

    for y in 0..IMG_SIZE
    {
        for x in 0..IMG_SIZE
        {
            let pokemon = &_model.battle.pokemons[y][x];
            let color: [u8; 4] = pokemon.kind.into();
            pixels.put_pixel(x as u32, y as u32, nannou::image::Rgba(color));
        }
    }

    let image = nannou::image::DynamicImage::ImageRgba8(pixels);
    let texture = wgpu::Texture::from_image(_app, &image);

    frame.clear(PURPLE);

    let draw = _app.draw();
    draw.texture(&texture);
    draw.to_frame(_app, &frame).unwrap();
}