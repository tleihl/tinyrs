use std::error::Error;
use std::path::Path;

use clap::Parser;

use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::event::Event;
use tinyrs::canvas::CanvasBuilder;
use tinyrs::common::Resolution;
use tinyrs::renderer::Renderer;
use tinyrs::model::Model;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    file: std::path::PathBuf,

    #[arg(long, default_value_t = 1024)]
    width: u32,

    #[arg(long, default_value_t = 768)]
    height: u32,
}

fn app<P: AsRef<Path>>(filename: P, resolution: Resolution) -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl2::init()?;
    let mut canvas = CanvasBuilder::new(&sdl_context)
        .resolution(resolution)
        .title("TinyRS")
        .build()?;

    let model = Model::from_file(filename)?;

    let renderer = Renderer::new(resolution);
    let mut zbuffer = vec![f64::MIN; (resolution.width * resolution.height) as usize];

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for face in model.iter() {
            renderer.render_face(&mut canvas, &mut zbuffer, face)?;
        }

        zbuffer.fill(f64::MIN);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        canvas.present();
    }

    Ok(())
}

pub fn main() {
    let args = Args::parse();
    app(args.file, (args.width, args.height).into())
        .map_err(|e| eprintln!("{}", e))
        .ok();
}