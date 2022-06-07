use clap::Parser;
use nannou::image::GenericImageView;
use nannou::prelude::{App, Frame, Update, WindowEvent};
use once_cell::sync::OnceCell;
use poke_fighting_rust::{
    Args, Battle, Colored, Fighter, FighterType, GenerateRandomly, Pokemon, SelectionAlgorithm,
    StreetFighter, RPS,
};
use std::fs::File;
use std::io;
use std::path::PathBuf;

// Needed because of nannou's not so great model function pointer
static ARGS: OnceCell<Args> = OnceCell::new();

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct AllArgs {
    #[clap(flatten)]
    args: Args,
    /// Config JSON file to use. When passed, overrides all command line arguments
    #[clap(short = 'c', long)]
    config: Option<PathBuf>,
}

fn parse_args() -> io::Result<&'static Args> {
    let all_args = AllArgs::parse();
    let args = if let Some(config) = all_args.config {
        let file = File::open(config)?;
        let reader = io::BufReader::new(file);
        serde_json::from_reader(reader)?
    } else {
        all_args.args
    };
    Ok(ARGS.get_or_init(|| args))
}

fn main() -> io::Result<()> {
    let args = parse_args()?;
    match args.fighter_type {
        FighterType::Pokemon => run_app::<Pokemon>(),
        FighterType::RockPaperScissors => run_app::<RPS>(),
        FighterType::StreetFighter => run_app::<StreetFighter>(),
    };
    Ok(())
}

fn run_app<T>()
where
    T: 'static + Colored + Fighter + GenerateRandomly,
{
    nannou::app(model::<T>).update(update).exit(exit).run()
}

struct Model<T> {
    battle: Battle<T>,
    image: nannou::image::DynamicImage,
    window_width: u32,
    window_height: u32,
    display_framerate: bool,
}

fn model<T: 'static + Fighter + GenerateRandomly>(app: &App) -> Model<T> {
    let args = ARGS.get().unwrap();
    let img_width = args.width;
    let img_height = args.height;
    let selection_algorithm = if args.random {
        SelectionAlgorithm::RandomNeighbour
    } else {
        SelectionAlgorithm::WeakestNeighbour
    };

    let surface_conf_builder = nannou::window::SurfaceConfigurationBuilder::new()
        .present_mode(nannou::wgpu::PresentMode::Mailbox);
    app.new_window()
        .size(img_width as u32, img_height as u32)
        .surface_conf_builder(surface_conf_builder)
        .clear_color(nannou::color::PURPLE)
        .view(view::<T>)
        .event(event::<T>)
        .build()
        .unwrap();

    Model {
        battle: Battle::new(img_width, img_height, selection_algorithm, !args.fightown),
        image: nannou::image::DynamicImage::ImageRgb8(nannou::image::RgbImage::new(
            img_width as u32,
            img_height as u32,
        )),
        window_width: img_width as u32,
        window_height: img_height as u32,
        display_framerate: args.framerate,
    }
}

fn update<T: Fighter + Colored>(_app: &App, model: &mut Model<T>, _update: Update) {
    model.battle.action();

    if let nannou::image::DynamicImage::ImageRgb8(ref mut pixels) = model.image {
        for (x, y, pixel) in pixels.enumerate_pixels_mut() {
            if let Some(fighter) = model.battle.fighter((x as usize, y as usize)) {
                *pixel = fighter.color();
            } else {
                *pixel = [0, 0, 0].into()
            }
        }
    }
}

fn view<T>(app: &App, model: &Model<T>, frame: Frame) {
    let texture = nannou::wgpu::Texture::from_image(app, &model.image);

    let (image_width, image_height) = model.image.dimensions();
    let width_ratio = model.window_width as f32 / image_width as f32;
    let height_ratio = model.window_height as f32 / image_height as f32;
    let ratio = if width_ratio < height_ratio {
        width_ratio
    } else {
        height_ratio
    };

    let draw = app.draw();
    if ratio == 1.0 {
        draw.texture(&texture);
    } else {
        draw.texture(&texture)
            .width(ratio * image_width as f32)
            .height(ratio * image_height as f32);
    }
    draw.to_frame(app, &frame).unwrap();

    if model.display_framerate && app.elapsed_frames() % 100 == 99 {
        println!("Frame rate: {:.2}", app.fps());
    }
}

fn event<T>(app: &App, model: &mut Model<T>, ev: nannou::event::WindowEvent) {
    match &ev {
        WindowEvent::KeyPressed(nannou::event::Key::Space) => {
            if let nannou::app::LoopMode::RefreshSync = app.loop_mode() {
                app.set_loop_mode(nannou::app::LoopMode::loop_ntimes(0));
            } else {
                app.set_loop_mode(nannou::app::LoopMode::refresh_sync());
            }
        }
        WindowEvent::Resized(resized) => {
            model.window_width = resized.x as u32;
            model.window_height = resized.y as u32;
        }
        _ => {}
    }
}

fn exit<T>(_app: &App, _model: Model<T>) {}
