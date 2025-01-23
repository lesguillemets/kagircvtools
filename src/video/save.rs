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
/// * `&[(usize, Mat)]`: 保存したい [Mat] と，それに振るべき番号の列
/// * dir: 保存先のディレクトリ
pub fn save_mats_as(stg: &SpawnSettings, imgs: &[(usize, Mat)]) {
    // TODO: 桁数
    let dir = PathBuf::from(&stg.dir);
    if !dir.is_dir() {
        panic!("video::base::save_mats_as: dir not directory");
    }
    let base = &stg.base_name;
    let ext = &stg.ext;
    for (serial, im) in imgs.iter() {
        let file = dir.join(format!("{base}_fr{serial:04}.{ext}"));
        save_mat_to(file.to_str().unwrap(), im);
    }
}

#[derive(Debug)]
/// 連番でできるたくさんのファイルを保存するような場合の設定
pub struct SpawnSettings {
    base_name: String,
    dir: String,
    ext: String,
}
