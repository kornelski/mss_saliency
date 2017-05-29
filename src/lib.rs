//! Sailiency map is computed using Maximum Symmetric Surround algorithm by Radhakrishna Achanta.
//!
//! http://ivrl.epfl.ch/research/saliency/MSSS.html

extern crate imgref;

mod integral;

use std::cmp::min;
use integral::*;

pub use imgref::{ImgRef, Img};

/// Create a saliency map.
///
/// The input is a single channel 2D 8-bit image (see imgref crate). The maximum image size is about 16Mpix, because internal u32 counters will start overflowing on larger images.
///
/// The output is a 2D array of the same size of `u16` values. Max value is 65025 (255*255), but expect most values to be low.
pub fn maximum_symmetric_surround_saliency(image: ImgRef<u8>) -> Img<Vec<u16>> {
    let integral_img = integral_image(image);

    let (width, height) = (image.width, image.height);
    let stride = image.stride as u32;

    let mut sal_map = Vec::with_capacity(width as usize * height as usize);
    for y in 0..height {
        let y_size = min(y, height - y);
        let y1 = y.saturating_sub(y_size);
        let y2 = min(y + y_size, height - 1);

        for x in 0..width {
            let x_size = min(x, width - x);
            let x1 = x.saturating_sub(x_size);
            let x2 = min(x + x_size, width - 1);

            let area = (x2 - x1 + 1) * (y2 - y1 + 1);

            let avg = (integral_img.integral_sum(x1, y1, x2, y2) / area as u32) as u8;
            let diff = (avg as i16 - image.buf[(y*stride + x) as usize] as i16) as i32;

            sal_map.push((diff*diff) as u16);
        }
    }

    return Img::new(sal_map, width as usize, height as usize);
}
