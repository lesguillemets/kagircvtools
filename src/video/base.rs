use opencv::core::Vector;
use opencv::imgcodecs::{imwrite, ImwriteFlags};
use opencv::prelude::*;

pub fn save_mat_to(filename: &str, img: &Mat) -> () {
    let flags: Vector<i32> = Vector::from_slice(&[ImwriteFlags::IMWRITE_PNG_COMPRESSION.into(), 9]);
    imwrite(filename, img, &flags).unwrap();
}
