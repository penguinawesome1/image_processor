use crate::image_data::{channel::Channel, pixel::Pixel};
use crate::input_reader::ReadError;
use itertools::iproduct;
use std::fs::{read, write};
use std::path::PathBuf;

const TGA_HEADER_LEN: usize = 18;

pub struct Image {
    data: Vec<Vec<Pixel>>,
}

impl Image {
    pub fn new(path: &PathBuf) -> Result<Image, ReadError> {
        let mut img: Image = Image { data: vec![] };
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
        iproduct!(0..self.height(), 0..self.width()).map(|(x, y)| (x, y, self.pixel(x, y)))
    }

    pub fn for_pixel<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, usize, &mut Pixel),
    {
        iproduct!(0..self.height(), 0..self.width())
            .for_each(|(x, y)| f(x, y, self.pixel_mut(x, y)));
    }

    // credit:
    // https://users.rust-lang.org/t/reading-binary-files-a-trivial-program-not-so-trivial-for-me/56166/3
    #[must_use]
    pub fn deserialize(&mut self, input_path: &PathBuf) -> Result<(), ReadError> {
        let Ok(bytes) = read(input_path) else {
            return Err(ReadError::MissingFile);
        };

        let w: usize = u16::from_le_bytes([bytes[12], bytes[13]]) as usize;
        let h: usize = u16::from_le_bytes([bytes[14], bytes[15]]) as usize;

        self.data = vec![vec![Pixel::default(); w]; h];

        self.for_pixel(|x, y, pixel_mut| {
            let byte_index: usize = TGA_HEADER_LEN + (x + y * w) * 3;

            *pixel_mut = Pixel::new(
                Channel(bytes[byte_index]),
                Channel(bytes[byte_index + 1]),
                Channel(bytes[byte_index + 2]),
            );
        });

        Ok(())
    }

    // credit:
    // https://users.rust-lang.org/t/how-to-write-16-bytes-to-a-file/89315/3
    #[must_use]
    pub fn serialize(
        &mut self,
        output_path: &PathBuf,
        input_path: &PathBuf,
    ) -> Result<(), ReadError> {
        let Ok(bytes) = read(input_path) else {
            return Err(ReadError::MissingFile);
        };

        let mut pixel_data: Vec<u8> = bytes[0..TGA_HEADER_LEN].to_vec();
        pixel_data.reserve(self.width() * self.height() * 3);

        self.for_pixel(|_, _, pixel_mut| {
            pixel_data.push(pixel_mut.b.0);
            pixel_data.push(pixel_mut.g.0);
            pixel_data.push(pixel_mut.r.0);
        });

        write(output_path, pixel_data).map_err(|_| ReadError::InvalidFileName)?;

        Ok(())
    }
}
