use opencv::core::Vector;
use opencv::imgcodecs::{imwrite, ImwriteFlags};
use opencv::prelude::*;
use std::path::PathBuf;

/// [Mat] を filename に保存する．
/// [imwrite] 呼んでるだけなので形式はファイル名からの推論
pub fn save_mat_to(filename: &str, img: &Mat) -> () {
    let flags: Vector<i32> = Vector::from_slice(&[ImwriteFlags::IMWRITE_PNG_COMPRESSION.into(), 9]);
    imwrite(filename, img, &flags).unwrap();
}

/// * 一連のフレームを`{base_name}_fr{serial}.{ext}` で保存
/// * `&[Mat]`: 保存したい [Mat] の列
/// * dir: 保存先のディレクトリ
/// * serial_start (defaults to 0): 連番の始まり
pub fn save_mats_as(
    base_name: &str,
    dir: &str,
    ext: &str,
    imgs: &[Mat],
    serial_start: Option<usize>,
) {
    // TODO: 桁数
    let mut dir = PathBuf::from(dir);
    if !dir.is_dir() {
        panic!("video::base::save_mats_as: dir not directory");
    }
    let serial_start = serial_start.unwrap_or(0);
    for (i, im) in imgs.iter().enumerate() {
        let serial = i + serial_start;
        let file = dir.join(format!("{base_name}_fr{serial:04}.{ext}"));
        save_mat_to(file.to_str().unwrap(), im);
    }
}
