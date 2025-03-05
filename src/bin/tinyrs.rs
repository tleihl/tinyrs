use std::error::Error;
use std::path::Path;

use clap::Parser;

use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::event::Event;
use tinyrs::canvas::CanvasBuilder;
use tinyrs::common::Resolution;
use tinyrs::geometry::{Mat4x4f, Vec3f};
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

    let light_direction = Vec3f::new(0.0, 0.0, 1.0);

    let mut camera = Vec3f::new(0.0,0.0,3.0);

    let view_port = Mat4x4f::viewport(
        resolution.width as f64 / 8.0,
        resolution.height as f64 / 8.0,
        resolution.width as f64 * 3.0 / 4.0,
        resolution.height as f64 * 3.0 / 4.0
    );

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        let projection = Mat4x4f::from([
            1.0, 0.0,  0.0,            0.0,
            0.0, 1.0,  0.0,            0.0,
            0.0, 0.0,  1.0,            0.0,
            0.0, 0.0, -1.0 / camera.z, 1.0,
        ]);

        for face in model.iter() {
            renderer.render_face(&mut canvas, &mut zbuffer, &light_direction,
                                 face, view_port, projection)?;
        }

        zbuffer.fill(f64::MIN);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseWheel { y, .. } => {
                    camera.z += 0.25 * y.signum() as f64;
                    camera.z = f64::clamp(camera.z, 2.0, 5.0);
                }
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