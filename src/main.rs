use clap::Parser;
use nannou::prelude::{App, Frame, Update, WindowEvent};
use nannou::image::GenericImageView;
use poke_fighting_rust::{Battle, Colored, Fighter, FighterType, Pokemon, RandomlyGeneratable, RPS, SelectionAlgorithm};

/// Battle simulation
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args
{
    /// Fighter type, either pokemon or rps
    #[clap(short='t', long, default_value_t = FighterType::Pokemon)]
    fighter_type: FighterType,

    /// Image width
    #[clap(short='w', long, default_value_t = 512, validator = validate_size)]
    width: usize,

    /// Image height
    #[clap(short='h', long, default_value_t = 512, validator = validate_size)]
    height: usize,

    /// When fighting, select random neighbour instead of the weakest one
    #[clap(short='r', long)]
    random: bool,

    /// Measure frame rate and print it to stdout
    #[clap(short='f', long)]
    framerate: bool,

    /// Let fighters fight their own kind
    #[clap(short='o', long)]
    fightown: bool,
}

fn main()
{
    let args = Args::parse();
    match args.fighter_type
    {
        FighterType::Pokemon => nannou::app(model::<Pokemon>).update(update).exit(exit).run(),
        FighterType::RPS => nannou::app(model::<RPS>).update(update).exit(exit).run(),
    };
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

struct Model<T>
{
    battle: Battle<T>,
    image: nannou::image::DynamicImage,
    window_width: u32,
    window_height: u32,
    measure_framerate: bool,
    measure_start: std::time::Instant,
    counter: usize,
}

fn model<T: 'static + Fighter + RandomlyGeneratable>(app: &App) -> Model<T>
{
    let args = Args::parse();
    let img_width = args.width;
    let img_height = args.height;
    let selection_algorithm = if args.random { SelectionAlgorithm::RandomNeighbour } else { SelectionAlgorithm::WeakestNeighbour };

    let surface_conf_builder = nannou::window::SurfaceConfigurationBuilder::new().present_mode(nannou::wgpu::PresentMode::Mailbox);
    app.new_window()
       .size(img_width as u32, img_height as u32)
       .surface_conf_builder(surface_conf_builder)
       .clear_color(nannou::color::PURPLE)
       .view(view::<T>)
       .event(event::<T>)
       .build()
       .unwrap();

    Model {
        battle: Battle::new(T::generate_randomly(), img_width, img_height, selection_algorithm, !args.fightown),
        image: nannou::image::DynamicImage::ImageRgb8(nannou::image::RgbImage::new(img_width as u32, img_height as u32)),
        window_width: img_width as u32,
        window_height: img_height as u32,
        measure_framerate: args.framerate,
        measure_start: std::time::Instant::now(),
        counter: 0,
    }
}

fn update<T: Fighter + Colored>(_app: &App, model: &mut Model<T>, _update: Update)
{
    model.battle.action();

    if let nannou::image::DynamicImage::ImageRgb8(ref mut pixels) = model.image
    {
        for (x, y, pixel) in pixels.enumerate_pixels_mut()
        {
            let fighter = model.battle.fighter(x, y);
            *pixel = fighter.color();
        }
    }

    if model.measure_framerate
    {
        const MEASURE_FRAMES_COUNT: usize = 100;

        model.counter += 1;
        if model.counter % MEASURE_FRAMES_COUNT == 0
        {
            let framerate = MEASURE_FRAMES_COUNT as f32 / model.measure_start.elapsed().as_secs_f32();
            println!("Frame rate: {:.2}", framerate);
            model.measure_start = std::time::Instant::now();
        }
    }
}

fn view<T>(app: &App, model: &Model<T>, frame: Frame)
{
    let texture = nannou::wgpu::Texture::from_image(app, &model.image);
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

fn event<T>(app: &App, model: &mut Model<T>, ev: nannou::event::WindowEvent)
{
    // todo turn into a match
    if let WindowEvent::KeyPressed(nannou::event::Key::Space) = &ev
    {
        if let nannou::app::LoopMode::RefreshSync = app.loop_mode()
        {
            app.set_loop_mode(nannou::app::LoopMode::loop_ntimes(0));
        }
        else
        {
            app.set_loop_mode(nannou::app::LoopMode::refresh_sync());
        }
    }

    if let WindowEvent::Resized(resized) = &ev
    {
        model.window_width = resized.x as u32;
        model.window_height = resized.y as u32;
    }
}


fn exit<T>(_app: &App, _model: Model<T>)
{

}
