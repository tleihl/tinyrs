use sdl2::render::WindowCanvas;
use sdl2::video::{Window, WindowBuildError};
use sdl2::{IntegerOrSdlError, Sdl, VideoSubsystem};

use crate::common::Resolution;
use crate::errors::RenderError;

pub struct CanvasBuilder<'a> {
    context: &'a Sdl,
    title: String,
    is_fullscreen: bool,
    resolution: Resolution,
}

impl<'a> CanvasBuilder<'a> {
    pub fn new(sdl: &'a Sdl) -> Self {
        CanvasBuilder {
            context: sdl,
            title: String::new(),
            is_fullscreen: false,
            resolution: Default::default(),
        }
    }

    pub fn title(&mut self, title: &str) -> &mut Self {
        self.title = String::from(title);
        self
    }

    pub fn fullscreen(&mut self, is_fullscreen: bool) -> &mut Self {
        self.is_fullscreen = is_fullscreen;
        self
    }

    pub fn resolution<R: Into<Resolution>>(&mut self, resolution: R) -> &mut Self {
        self.resolution = resolution.into();
        self
    }

    pub fn build(&self) -> Result<WindowCanvas, RenderError> {
        let video_subsystem = self.context.video().map_err(|e| {
            RenderError::ContextError(e)
        })?;
        let window = self.build_window(video_subsystem)?;
        let canvas = self.build_canvas(window)?;
        Ok(canvas)
    }

    fn build_window(&self, video_subsystem: VideoSubsystem) -> Result<Window, WindowBuildError> {
        if self.is_fullscreen {
            video_subsystem
                .window(&self.title, 0, 0)
                .fullscreen_desktop()
                .build()
        } else {
            video_subsystem
                .window(&self.title, self.resolution.width, self.resolution.height)
                .build()
        }
    }

    fn build_canvas(&self, window: Window) -> Result<WindowCanvas, IntegerOrSdlError> {
        window.into_canvas().accelerated().build()
    }
}
