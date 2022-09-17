#![feature(test)]
extern crate test;


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
        maximum_symmetric_surround_saliency(test::black_box(img.as_ref()))
            .pixels().map(|v|v as u32).sum::<u32>()
    });
}

#[bench]
fn map_sum_rgb(b: &mut Bencher) {
    let img = ImgVec::new(vec![RGB::new(127u8,123,255); 640*480], 640, 480);
    b.iter(|| {
        maximum_symmetric_surround_saliency_rgb(test::black_box(img.as_ref()))
            .pixels().map(|v|v as u32).sum::<u32>()
    });
}
