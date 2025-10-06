use crate::image_data::channel::Channel;
use crate::image_data::normal_pixel::NormalPixel;
use std::ops;

#[derive(Debug, Clone, Copy, Default)]
pub struct Pixel {
    pub b: Channel,
    pub g: Channel,
    pub r: Channel,
}

impl Pixel {
    pub fn new(b: Channel, g: Channel, r: Channel) -> Self {
        Pixel { b, g, r }
    }

    pub fn from_normal(n: NormalPixel) -> Self {
        Pixel::new(
            Channel::from_normal(n.b),
            Channel::from_normal(n.g),
            Channel::from_normal(n.r),
        )
    }

    pub fn to_normal(&self) -> NormalPixel {
        NormalPixel::new(self.b.to_normal(), self.g.to_normal(), self.r.to_normal())
    }
}

impl ops::Add for Pixel {
    type Output = Pixel;

    fn add(self, other: Pixel) -> Pixel {
        Pixel::new(self.b + other.b, self.g + other.g, self.r + other.r)
    }
}

impl ops::Sub for Pixel {
    type Output = Pixel;

    fn sub(self, other: Pixel) -> Pixel {
        Pixel::new(self.b - other.b, self.g - other.g, self.r - other.r)
    }
}

impl ops::Mul for Pixel {
    type Output = Pixel;

    fn mul(self, other: Pixel) -> Pixel {
        Pixel::new(self.b * other.b, self.g * other.g, self.r * other.r)
    }
}

impl ops::Add<u8> for Pixel {
    type Output = Pixel;

    fn add(self, n: u8) -> Pixel {
        Pixel::new(self.b + n, self.g + n, self.r + n)
    }
}

impl ops::Sub<u8> for Pixel {
    type Output = Pixel;

    fn sub(self, n: u8) -> Pixel {
        Pixel::new(self.b - n, self.g - n, self.r - n)
    }
}

impl ops::Mul<f32> for Pixel {
    type Output = Pixel;

    fn mul(self, n: f32) -> Pixel {
        Pixel::new(self.b * n, self.g * n, self.r * n)
    }
}
