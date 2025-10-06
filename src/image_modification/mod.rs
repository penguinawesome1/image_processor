mod adjuster;

use crate::image_data::image::Image;
use crate::image_modification::adjuster::*;
use crate::input_reader::{ReadError, next_img, next_num};

#[must_use]
pub fn apply_method(img: &mut Image, args: &mut Vec<String>, arg: &str) -> Result<(), ReadError> {
    match arg {
        "multiply" => multiply(img, &next_img(args)?),
        "subtract" => subtract(img, &next_img(args)?),
        "overlay" => overlay(img, &next_img(args)?),
        "screen" => screen(img, &next_img(args)?),
        "combine" => combine(img, &next_img(args)?, &next_img(args)?),
        "flip" => img.flip(),
        "onlyred" => only_red(img),
        "onlygreen" => only_green(img),
        "onlyblue" => only_blue(img),
        "addred" => add_red(img, next_num(args)? as u8),
        "addgreen" => add_green(img, next_num(args)? as u8),
        "addblue" => add_blue(img, next_num(args)? as u8),
        "scalered" => scale_red(img, next_num(args)? as f32),
        "scalegreen" => scale_green(img, next_num(args)? as f32),
        "scaleblue" => scale_blue(img, next_num(args)? as f32),
        _ => return Err(ReadError::InvalidMethodName),
    }

    Ok(())
}
