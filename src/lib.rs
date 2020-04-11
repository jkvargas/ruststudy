pub mod renderer;

#[derive(Debug, PartialEq)]
pub enum Error {
    Render(String)
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<shaderc::Error> for Error {
    fn from(error: shaderc::Error) -> Self {
        Error::Render(error.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Render(error.to_string())
    }
}