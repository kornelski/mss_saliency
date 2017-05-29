use std::iter::repeat;
use imgref::*;

pub fn integral_image(image: ImgRef<u8>) -> ImgVec<u32> {
    let (in_width, in_height) = (image.width(), image.height());
    let stride = image.stride();

    let out_width = in_width + 1;
    let out_height = in_height + 1;

    let mut out = Vec::with_capacity(out_width * out_height);
    out.extend(repeat(0u32).take(out_width));

    for y in 0..in_height {
        let mut sum = 0;
        out.push(0);
        for x in 1..out_width {
            sum += image.buf[y*stride + (x - 1)] as u32;
            let above = out[y*out_width + x];
            out.push(above + sum);
        }
    }

    ImgVec::new(out, out_width, out_height)
}

pub trait IntegralSum {
    fn integral_sum(&self, x1: u32, y1: u32, x2: u32, y2: u32) -> u32;
}

impl IntegralSum for ImgVec<u32> {
    #[inline]
    fn integral_sum(&self, x1: u32, y1: u32, x2: u32, y2: u32) -> u32 {
        let stride = self.stride as u32;
        let x2_y2 = self.buf[(y2*stride + x2) as usize];

        if x1 == 0 && y1 == 0 {
            x2_y2
        } else if x1 == 0 {
            x2_y2 - self.buf[((y1 - 1)*stride + x2) as usize]
        } else if y1 == 0 {
            x2_y2 - self.buf[(y2*stride + x1 - 1) as usize]
        } else {
            x2_y2 + self.buf[((y1 - 1)*stride + x1 - 1) as usize]
                  - self.buf[((y1 - 1)*stride + x2) as usize]
                  - self.buf[(y2*stride + x1 - 1) as usize]
        }
    }
}
