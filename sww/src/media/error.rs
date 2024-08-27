use image::ImageError;
use std::io;

pub type Result<T> = std::result::Result<T, ReadImageError>;

#[derive(Debug)]
pub enum ReadImageError {
    IO(io::Error),
    Image(ImageError),
}

impl From<io::Error> for ReadImageError {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<ImageError> for ReadImageError {
    fn from(value: ImageError) -> Self {
        Self::Image(value)
    }
}
