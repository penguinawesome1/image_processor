use std::ops;

#[derive(Debug, Clone, Copy, Default)]
pub struct NormalPixel {
    pub b: f32,
    pub g: f32,
    pub r: f32,
}

impl NormalPixel {
    pub fn new(b: f32, g: f32, r: f32) -> Self {
        Self { b, g, r }
    }

    pub fn invert(&self) -> Self {
        Self::new(1.0 - self.b, 1.0 - self.g, 1.0 - self.r)
    }
}

impl ops::Mul for NormalPixel {
    type Output = NormalPixel;

    fn mul(self, other: NormalPixel) -> NormalPixel {
        NormalPixel::new(self.b * other.b, self.g * other.g, self.r * other.r)
    }
}

impl ops::Mul<f32> for NormalPixel {
    type Output = NormalPixel;

    fn mul(self, n: f32) -> NormalPixel {
        NormalPixel::new(self.b * n, self.g * n, self.r * n)
    }
}
