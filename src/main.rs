use rust::image_modification::apply_method;
use rust::input_reader::{ReadError, read_input};
use std::env;

fn main() -> Result<(), ReadError> {
    let args: Vec<String> = env::args().collect();
    let args_len: usize = args.len();
    let (o_path, i_path, mut img) = read_input(&args)?;

    for i in 3..args_len {
        apply_method(&mut img, &args, i)?;
    }

    img.serialize(&o_path, &i_path);

    Ok(())
}
