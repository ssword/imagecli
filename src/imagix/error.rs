use image::error;
use std::convert::From;
use std::{fmt, io};

#[derive(Debug)]
pub enum ImagixError {
    FileIOError(String),
    UserInputError(String),
    ImageResizingError(String),
    FormatError(String),
}

