#![feature(test)]
extern crate test;
extern crate mss_saliency;

use mss_saliency::*;
use test::Bencher;

#[bench]
fn new_map(b: &mut Bencher) {
    let img = ImgVec::new(vec![127u8; 640*480], 640, 480);
    b.iter(|| {
        maximum_symmetric_surround_saliency(test::black_box(img.as_ref()))
    });
}

#[bench]
fn map_sum(b: &mut Bencher) {
    let img = ImgVec::new(vec![127u8; 640*480], 640, 480);
    b.iter(|| {
        let s:u32 = maximum_symmetric_surround_saliency(test::black_box(img.as_ref())).buf.into_iter().map(|v|v as u32).sum();
        s
    });
}
