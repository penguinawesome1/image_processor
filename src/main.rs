use rust::image_modification::apply_method;
use rust::input_reader::{ReadError, read_input};
use std::env;

fn image_processor() -> Result<(), ReadError> {
    let mut args: Vec<String> = env::args().skip(1).rev().collect();

    let (o_path, i_path, mut img) = read_input(&mut args)?;

    while let Some(arg) = args.pop() {
        apply_method(&mut img, &mut args, &arg)?;
    }

    img.serialize(&o_path, &i_path)?;

    Ok(())
}

fn main() {
    if let Err(e) = image_processor() {
        eprintln!("{}", e);
    }
}
