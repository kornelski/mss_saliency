use imgref::{ImgRef, ImgVec};
use std::iter::repeat;
use std::ops::{Add, AddAssign, Sub};

pub struct IntegralImage<Sums>(ImgVec<Sums>);

impl<Sums> IntegralImage<Sums> {
    pub fn new<Input>(image: ImgRef<'_, Input>) -> Self
        where Input: Copy, Sums: From<Input> + Default + AddAssign + Copy + Add<Output = Sums>
    {
        let (in_width, in_height) = (image.width(), image.height());

        let out_width = in_width + 1;
        let out_height = in_height + 1;

        let mut out = Vec::with_capacity(out_width * out_height);
        out.extend(repeat(Sums::default()).take(out_width));

        for (y, row) in image.rows().enumerate() {
            let mut sum = Sums::default();
            out.push(Sums::default());
            for (x, px) in row.iter().copied().enumerate() {
                sum += Sums::from(px);
                let above = out[y*out_width + x+1];
                out.push(above + sum);
            }
        }

        Self(ImgVec::new(out, out_width, out_height))
    }

    #[inline]
    pub fn integral_sum(&self, x1: u32, y1: u32, x2: u32, y2: u32) -> Sums
        where Sums: Copy + Sub<Output=Sums> + Add<Output=Sums> {
        let x2_y2 = self.0[(x2, y2)];

        if x1 == 0 && y1 == 0 {
            x2_y2
        } else if x1 == 0 {
            x2_y2 - self.0[(x2, y1 - 1)]
        } else if y1 == 0 {
            x2_y2 - self.0[(x1 - 1, y2)]
        } else {
            x2_y2 + self.0[(x1 - 1, y1 - 1)]
                  - self.0[(x2, y1 - 1)]
                  - self.0[(x1 - 1, y2)]
        }
    }
}
