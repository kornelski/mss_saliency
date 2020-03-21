
use std::env;
use std::cmp;
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from(env::args_os().nth(1).expect("Please provide a PNG file as an argument"));
    let img = lodepng::decode32_file(&path).unwrap();
    let fast_gray: Vec<_> = img.buffer.iter().map(|px| {
        (px.a as u32 * (px.r as u32 + px.g as u32 * 2 + px.b as u32) / (4*256)) as u8
    }).collect();

    let gray_img = imgref::ImgRef::new(&fast_gray, img.width, img.height);

    let sal = mss_saliency::maximum_symmetric_surround_saliency(gray_img);

    let rgb: Vec<_> = sal.pixels().map(|v| (cmp::min(255,v>>6) as u8,(v>>8) as u8,cmp::min(255,v>>2) as u8)).collect();
    let new_path = format!("{}-mss-saliency.png", path.display());
    lodepng::encode24_file(&new_path, &rgb, sal.width(), sal.height()).unwrap();
    println!("Wrote {}", new_path);
}
