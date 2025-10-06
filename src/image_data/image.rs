use crate::image_data::pixel::Pixel;
use crate::input_reader::ReadError;
use itertools::iproduct;
use std::path::PathBuf;

pub struct Image {
    data: Vec<Vec<Pixel>>,
}

impl Image {
    pub fn new(path: &PathBuf) -> Result<Image, ReadError> {
        let img: Image = Image { data: vec![] };
        img.deserialize(path)?;
        Ok(img)
    }
}

impl Image {
    pub fn width(&self) -> usize {
        if self.height() == 0 {
            return 0;
        }

        self.data[0].len()
    }

    pub fn height(&self) -> usize {
        self.data.len()
    }

    pub fn flip(&mut self) {
        self.data.reverse();

        for row in self.data.iter_mut() {
            row.reverse();
        }
    }

    pub fn pixel_mut(&mut self, x: usize, y: usize) -> &mut Pixel {
        &mut self.data[y][x]
    }

    pub fn pixel(&self, x: usize, y: usize) -> &Pixel {
        &self.data[y][x]
    }

    pub fn view_pixels(&self) -> impl Iterator<Item = (usize, usize, &Pixel)> {
        iproduct!(0..self.width(), 0..self.height()).map(|(x, y)| (x, y, self.pixel(x, y)))
    }

    pub fn for_pixel<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, usize, &mut Pixel),
    {
        iproduct!(0..self.width(), 0..self.height())
            .for_each(|(x, y)| f(x, y, self.pixel_mut(x, y)));
    }

    #[must_use]
    pub fn deserialize(&self, _input_path: &PathBuf) -> Result<(), ReadError> {
        unimplemented!();
    }

    pub fn serialize(&self, _output_path: &PathBuf, _input_path: &PathBuf) {
        unimplemented!();
    }
}
