use std::fmt;

# [derive(Debug)]
pub enum RenderError {
    SDLError(String),
    ContextError(String),
    WindowBuildError(sdl2::video::WindowBuildError),
    IntegerOrSdlError(sdl2::IntegerOrSdlError),
    FileReadError(std::io::Error),
    VertexParsingError(String),
    NormalParsingError(String),
    TextureParsingError(String),
    FaceParsingError(String),
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RenderError::SDLError(msg) =>
                write!(f, "SDL error: {}", msg),
            RenderError::ContextError(msg) =>
                write!(f, "Unable to create context: {}", msg),
            RenderError::WindowBuildError(err) =>
                write!(f, "Unable to build window: {}", err.to_string()),
            RenderError::IntegerOrSdlError(err) =>
                write!(f, "Unable to initialize SDL: {}", err.to_string()),
            RenderError::FileReadError(err) =>
                write!(f, "Unable to read file: {}", err.to_string()),
            RenderError::VertexParsingError(msg) =>
                write!(f, "Unable to parse vertex: {}", msg),
            RenderError::NormalParsingError(msg) =>
                write!(f, "Unable to parse normal: {}", msg),
            RenderError::TextureParsingError(msg) =>
                write!(f, "Unable to parse texture: {}", msg),
            RenderError::FaceParsingError(msg) =>
                write!(f, "Unable to parse face: {}", msg),
        }
    }
}

impl From<sdl2::video::WindowBuildError> for RenderError {
    fn from(err: sdl2::video::WindowBuildError) -> RenderError {
        RenderError::WindowBuildError(err)
    }
}

impl From<sdl2::IntegerOrSdlError> for RenderError {
    fn from(err: sdl2::IntegerOrSdlError) -> RenderError {
        RenderError::IntegerOrSdlError(err)
    }
}

impl From<std::io::Error> for RenderError {
    fn from(err: std::io::Error) -> RenderError {
        RenderError::FileReadError(err)
    }
}

impl From<String> for RenderError {
    fn from(msg: String) -> RenderError {
        RenderError::SDLError(msg)
    }
}

impl std::error::Error for RenderError {}
