use clap::Parser;
use core::fmt::Display;
use nannou::image::GenericImageView;
use nannou::prelude::{App, Frame, Update};
use nannou_egui::{egui, Egui};
use once_cell::sync::OnceCell;
use poke_fighting_rust::{
    Args, Battle, Colored, Fighter, FighterType, GenerateRandomly, Pokemon, SelectionAlgorithm,
    StreetFighter, RPS,
};
use std::cmp::min;
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
    T: 'static + Colored + Fighter + GenerateRandomly + Display,
{
    nannou::app(model::<T>).update(update).exit(exit).run()
}

struct Model<T> {
    battle: Battle<T>,
    image: nannou::image::DynamicImage,
    window_size: (u32, u32),
    paused: bool,
    display_framerate: bool,
    info: Egui,
    info_visible: bool,
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
    let window_id = app
        .new_window()
        .size(img_width as u32, img_height as u32)
        .surface_conf_builder(surface_conf_builder)
        .view(view::<T>)
        .key_pressed(key_pressed::<T>)
        .resized(resized::<T>)
        .mouse_pressed(mouse_pressed::<T>)
        .raw_event(raw_event::<T>)
        .build()
        .unwrap();

    let window = app.window(window_id).unwrap();

    Model {
        battle: Battle::new(img_width, img_height, selection_algorithm, !args.fightown),
        image: nannou::image::DynamicImage::ImageRgb8(nannou::image::RgbImage::new(
            img_width as u32,
            img_height as u32,
        )),
        window_size: (img_width as u32, img_height as u32),
        paused: false,
        display_framerate: args.framerate,
        info: Egui::from_window(&window),
        info_visible: false,
    }
}

fn update<T: Fighter + Colored + Display>(app: &App, model: &mut Model<T>, _update: Update) {
    if !model.paused {
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

    let window_size = model.window_size;
    let image_size = model.image.dimensions();
    let ctx = model.info.begin_frame();
    egui::Window::new("Fighter info")
        .resizable(false)
        .collapsible(false)
        .open(&mut model.info_visible)
        .show(&ctx, |ui| {
            let (x, y) =
                window_coords_to_image((app.mouse.x, app.mouse.y), window_size, image_size);
            ui.label(format!("Position: {}, {}", x, y));
            if let Some(fighter) = model.battle.fighter((x as usize, y as usize)) {
                let color = fighter.color();
                ui.horizontal(|ui| {
                    let size = ui.spacing().icon_width;
                    let (rect, _response) = ui
                        .allocate_exact_size(egui::Vec2 { x: size, y: size }, egui::Sense::hover());
                    ui.painter().circle_filled(
                        rect.center(),
                        size * 0.5,
                        egui::Color32::from_rgb(color[0], color[1], color[2]),
                    );

                    ui.label(format!("{}", fighter));
                });
            } else {
                ui.label("Nothing here");
            }
        });
}

fn get_image_ratio(
    (window_width, window_height): (u32, u32),
    (image_width, image_height): (u32, u32),
) -> f32 {
    let width_ratio = window_width as f32 / image_width as f32;
    let height_ratio = window_height as f32 / image_height as f32;
    if width_ratio < height_ratio {
        width_ratio
    } else {
        height_ratio
    }
}

fn window_coords_to_image(
    window_pos: (f32, f32),
    window_size: (u32, u32),
    image_size: (u32, u32),
) -> (u32, u32) {
    // Consider image scaling to convert window coordinates into image coordinates.
    let ratio = get_image_ratio(window_size, image_size);
    let image_pos = (window_pos.0 / ratio, window_pos.1 / ratio);

    // Coordinates are relative to the middle of the image, we want them relative to the upper-left
    // corner.
    let corner_relative_pos = (
        (image_size.0 as f32) * 0.5 + image_pos.0,
        (image_size.1 as f32) * 0.5 - image_pos.1,
    );
    let corner_relative_pos = (
        if corner_relative_pos.0 > 0.0 {
            corner_relative_pos.0 as u32
        } else {
            0
        },
        if corner_relative_pos.1 > 0.0 {
            corner_relative_pos.1 as u32
        } else {
            0
        },
    );

    // Make sure to correct out of bounds positions
    (
        min(corner_relative_pos.0, image_size.0 - 1),
        min(corner_relative_pos.1, image_size.1 - 1),
    )
}

fn view<T>(app: &App, model: &Model<T>, frame: Frame) {
    let texture = nannou::wgpu::Texture::from_image(app, &model.image);

    let ratio = get_image_ratio(model.window_size, model.image.dimensions());

    frame.clear(nannou::color::PURPLE);

    let draw = app.draw();
    if ratio == 1.0 {
        draw.texture(&texture);
    } else {
        draw.texture(&texture)
            .width(ratio * model.image.width() as f32)
            .height(ratio * model.image.height() as f32);
    }
    draw.to_frame(app, &frame).unwrap();

    model.info.draw_to_frame(&frame).unwrap();

    if model.display_framerate && app.elapsed_frames() % 100 == 99 {
        println!("Frame rate: {:.2}", app.fps());
    }
}

fn key_pressed<T>(app: &App, model: &mut Model<T>, key: nannou::event::Key) {
    if key == nannou::event::Key::Space {
        model.paused = !model.paused;
        if model.paused {
            app.set_loop_mode(nannou::app::LoopMode::wait());
        } else {
            app.set_loop_mode(nannou::app::LoopMode::refresh_sync());
        }
    }
}

fn resized<T>(_app: &App, model: &mut Model<T>, size: nannou::glam::Vec2) {
    model.window_size = (size.x as u32, size.y as u32);
}

fn mouse_pressed<T>(_app: &App, model: &mut Model<T>, button: nannou::event::MouseButton) {
    if button == nannou::event::MouseButton::Left {
        model.info_visible = true;
    }
}

fn raw_event<T>(_app: &App, model: &mut Model<T>, event: &nannou::winit::event::WindowEvent) {
    model.info.handle_raw_event(event);
}

fn exit<T>(_app: &App, _model: Model<T>) {}
