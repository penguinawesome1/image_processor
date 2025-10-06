use crate::image_data::image::Image;
use std::ffi::OsStr;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadError {
    #[error(
        "Project 1: Image Processing, Fall 2025\n\nUsage:\n\t./project1.out [output] [firstImage] [method] [...]"
    )]
    RequestHelp,
    #[error("Invalid file name.\n")]
    InvalidFileName,
    #[error("File does not exist.\n")]
    MissingFile,
    #[error("Invalid method name.\n")]
    InvalidMethodName,
    #[error("Missing argument.\n")]
    MissingArgument,
    #[error("Invalid argument, invalid file name.\n")]
    InvalidFileNameArg,
    #[error("Invalid argument, file does not exist.\n")]
    MissingFileArg,
    #[error("Invalid argument, expected number.\n")]
    MissingNumArg,
}

fn is_tga_path(path: &PathBuf) -> bool {
    path.extension() != Some(OsStr::new("tga"))
}

pub fn read_input(args: &[String]) -> Result<(PathBuf, PathBuf, Image), ReadError> {
    let args_len: usize = args.len();

    match args_len {
        1 => return Err(ReadError::RequestHelp),
        2 if args[1] == "--help" => return Err(ReadError::RequestHelp),
        2 => return Err(ReadError::InvalidFileName),
        3 => return Err(ReadError::InvalidMethodName),
        _ => (),
    };

    let output_path: PathBuf = PathBuf::from(&args[1]);
    let input_path: PathBuf = PathBuf::from(&args[2]);

    if !is_tga_path(&output_path) || !is_tga_path(&input_path) {
        return Err(ReadError::InvalidFileName);
    }

    let tracking_img: Image = Image::new(&input_path)?;

    Ok((output_path, input_path, tracking_img))
}

pub fn next_img(args_len: usize, args: &[String], index: usize) -> Result<Image, ReadError> {
    if index >= args_len {
        return Err(ReadError::MissingArgument);
    }

    let path: PathBuf = PathBuf::from(&args[index]);

    if !is_tga_path(&path) {
        return Err(ReadError::InvalidFileNameArg);
    }

    Ok(Image::new(&path)?)
}

pub fn next_num(args_len: usize, args: &[String], index: usize) -> Result<i32, ReadError> {
    if index >= args_len {
        return Err(ReadError::MissingArgument);
    }

    args[index].parse().map_err(|_| ReadError::MissingNumArg)
}
