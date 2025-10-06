use crate::image_data::{image::Image, normal_pixel::NormalPixel, pixel::Pixel};

macro_rules! image_operation {
    ($name: ident, $op: tt) => {
        pub fn $name(img1: &mut Image, img2: &Image) {
            img1.for_pixel(|x, y, pixel_mut| {
                *pixel_mut = *pixel_mut $op *img2.pixel(x, y);
            });
        }
    };
}

macro_rules! image_operation_channel {
    ($name: ident, $ty: tt, $op: tt, $channel: ident) => {
        pub fn $name(img1: &mut Image, num: $ty) {
            img1.for_pixel(|_, _, pixel_mut| {
                pixel_mut.$channel = pixel_mut.$channel $op num;
            });
        }
    };
}

macro_rules! image_only_channel {
    ($name: ident, $channel: ident) => {
        pub fn $name(img1: &mut Image) {
            img1.for_pixel(|_, _, pixel_mut| {
                pixel_mut.r = pixel_mut.$channel;
                pixel_mut.g = pixel_mut.$channel;
                pixel_mut.b = pixel_mut.$channel;
            });
        }
    };
}

image_operation!(multiply, *);
image_operation!(subtract, -);
image_operation_channel!(add_red, u8, +, r);
image_operation_channel!(add_green, u8, +, g);
image_operation_channel!(add_blue, u8, +, b);
image_operation_channel!(scale_red, f32, *, r);
image_operation_channel!(scale_green, f32, *, g);
image_operation_channel!(scale_blue, f32, *, b);
image_only_channel!(only_red, r);
image_only_channel!(only_green, g);
image_only_channel!(only_blue, b);

pub fn screen(img1: &mut Image, img2: &Image) {
    img1.for_pixel(|x, y, pixel_mut| {
        *pixel_mut = (pixel_mut.invert() * img2.pixel(x, y).invert()).invert();
    });
}

pub fn combine(red_img: &mut Image, green_img: &Image, blue_img: &Image) {
    red_img.for_pixel(|x, y, pixel_mut| {
        pixel_mut.g = green_img.pixel(x, y).g;
        pixel_mut.b = blue_img.pixel(x, y).b;
    });
}

pub fn overlay(img1: &mut Image, img2: &Image) {
    img1.for_pixel(|x, y, pixel_mut| {
        let other_norm: NormalPixel = img2.pixel(x, y).to_normal();

        let norm_overlay1: NormalPixel = pixel_mut.to_normal() * other_norm * 2.0;
        let overlay1: Pixel = Pixel::from_normal(norm_overlay1);

        let norm_overlay2_inverted: NormalPixel =
            pixel_mut.invert().to_normal() * img2.pixel(x, y).invert().to_normal() * 2.0;
        let overlay2: Pixel = Pixel::from_normal(norm_overlay2_inverted).invert();

        *pixel_mut = Pixel::new(
            if other_norm.b <= 0.5 {
                overlay1.b
            } else {
                overlay2.b
            },
            if other_norm.g <= 0.5 {
                overlay1.g
            } else {
                overlay2.g
            },
            if other_norm.r <= 0.5 {
                overlay1.r
            } else {
                overlay2.r
            },
        )
    });
}
