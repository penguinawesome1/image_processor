use crate::image_data::image::Image;
use std::ffi::OsStr;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadError {
    #[error(
        "Project 1: Image Processing, Fall 2025\n\nUsage:\n\tcargo run [output] [first_image] [method] [...]"
    )]
    RequestHelp,
    #[error("Invalid file name.")]
    InvalidFileName,
    #[error("File does not exist.")]
    MissingFile,
    #[error("Invalid method name.")]
    InvalidMethodName,
    #[error("Missing argument.")]
    MissingArgument,
    #[error("Invalid argument, invalid file name.")]
    InvalidFileNameArg,
    #[error("Invalid argument, file does not exist.")]
    MissingFileArg,
    #[error("Invalid argument, expected number.")]
    MissingNumArg,
}

fn is_tga_path(path: &PathBuf) -> bool {
    path.extension() == Some(OsStr::new("tga"))
}

pub fn read_input(args: &mut Vec<String>) -> Result<(PathBuf, PathBuf, Image), ReadError> {
    let args_len: usize = args.len();

    match args_len {
        0 => return Err(ReadError::RequestHelp),
        1 if args[0] == "help" => return Err(ReadError::RequestHelp),
        1 => return Err(ReadError::InvalidFileName),
        2 => return Err(ReadError::InvalidMethodName),
        _ => (),
    };

    let output_path: PathBuf = PathBuf::from(args.pop().unwrap());
    let input_path: PathBuf = PathBuf::from(args.pop().unwrap());

    if !is_tga_path(&output_path) || !is_tga_path(&input_path) {
        return Err(ReadError::InvalidFileName);
    }

    let tracking_img: Image = Image::new(&input_path)?;

    Ok((output_path, input_path, tracking_img))
}

pub fn next_img(args: &mut Vec<String>) -> Result<Image, ReadError> {
    let Some(arg) = args.pop() else {
        return Err(ReadError::MissingArgument);
    };

    let path: PathBuf = PathBuf::from(&arg);

    if !is_tga_path(&path) {
        return Err(ReadError::InvalidFileNameArg);
    }

    Ok(Image::new(&path)?)
}

pub fn next_num(args: &mut Vec<String>) -> Result<i32, ReadError> {
    let Some(arg) = args.pop() else {
        return Err(ReadError::MissingArgument);
    };

    arg.parse().map_err(|_| ReadError::MissingNumArg)
}
