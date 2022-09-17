//! Sailiency map is computed using Maximum Symmetric Surround algorithm by Radhakrishna Achanta.
//!
//! <https://core.ac.uk/download/pdf/147962379.pdf>


mod integral;
use std::ops::{Add, AddAssign, Sub};
use crate::integral::*;

pub use imgref::*;
pub use rgb::RGB;
use rgb::ComponentMap;

/// Create a saliency map.
///
/// The input is a single channel 2D 8-bit image (see imgref crate). The maximum image size is about 16Mpix, because internal u32 counters will start overflowing on larger images.
///
/// The output is a 2D array of the same size of `u16` values. Max value is 65025 (255*255), but expect most values to be low.
pub fn maximum_symmetric_surround_saliency(image: Img<&[u8]>) -> Img<Vec<u16>> {
    maximum_symmetric_surround_saliency_generic::<u8, u32>(image)
}

pub fn maximum_symmetric_surround_saliency_rgb(image: Img<&[RGB<u8>]>) -> Img<Vec<u16>> {
    maximum_symmetric_surround_saliency_generic::<RGB<u8>, RGB<u32>>(image)
}

fn maximum_symmetric_surround_saliency_generic<I, O>(image: Img<&[I]>) -> Img<Vec<u16>>
    where I: Copy + AreaDiff<O>, O: From<I> + Default + AddAssign + Copy + Add<Output=O> + Sub<Output=O> {
    let integral_img = IntegralImage::new(image);

    let (width, height) = (image.width() as u32, image.height() as u32);

    let mut sal_map = Vec::with_capacity(width as usize * height as usize);
    for y in 0..height {
        let y_size = y.min(height - y);
        let y1 = y.saturating_sub(y_size);
        let y2 = (y + y_size).min(height - 1);

        for x in 0..width {
            let x_size = x.min(width - x);
            let x1 = x.saturating_sub(x_size);
            let x2 = (x + x_size).min(width - 1);

            let area = (x2 - x1 + 1) * (y2 - y1 + 1);
            let diff = image[(x, y)].area_diff(integral_img.integral_sum(x1, y1, x2, y2), area);
            sal_map.push(diff.pow(2) as u16);
        }
    }

    Img::new(sal_map, width as usize, height as usize)
}

trait AreaDiff<T> {
    fn area_diff(self, sum: T, area: u32) -> i32;
}

impl AreaDiff<u32> for u8 {
    fn area_diff(self, sum: u32, area: u32) -> i32 {
        ((sum / area) as i16 - self as i16) as i32
    }
}

impl AreaDiff<RGB<u32>> for RGB<u8> {
    fn area_diff(self, sum: RGB<u32>, area: u32) -> i32 {
        let tmp = sum.map(|s| (s / area) as i16) - self.map(|c| c as i16);
        tmp.r.max(tmp.g).max(tmp.b) as i32
    }
}

#[test]
fn oversized_input_is_ok() {
    let img = ImgVec::new(vec![127u8; 35*18], 33, 17);
    let res = maximum_symmetric_surround_saliency(img.as_ref());
    assert_eq!(res.buf().len(), 33*17);
}
