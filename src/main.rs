use rust::image_modification::apply_method;
use rust::input_reader::{ReadError, read_input};
use std::env;

fn image_processor() -> Result<(), ReadError> {
    let args: Vec<String> = env::args().collect();
    let args_len: usize = args.len();
    let (o_path, i_path, mut img) = read_input(&args)?;

    let mut i: usize = 3;
    while i < args_len {
        apply_method(&mut img, &args, &mut i)?;
        i += 1;
    }

    img.serialize(&o_path, &i_path)?;

    Ok(())
}

fn main() {
    if let Err(e) = image_processor() {
        eprintln!("{}", e);
    }
}
