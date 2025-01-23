use opencv::prelude::*;
use opencv::videoio::VideoCapture;

use crate::video::load::load_video;
use crate::video::save::{save_mats_as, SpawnSettings};

/// from-th frame から to-th frame まで (to-th は含まない) をとる
///
/// * 途中でフレームが空になっても許容することにするが，そこは飛ばす
/// * returns: [opencv::error::Result]<[Vec]<[Mat]>>
///   [VideoCaptureTrait::read](opencv::prelude::VideoCaptureTrait::read) が Err を返すときに同様にそれを返す
pub fn get_frame_fromto(
    vc: &mut VideoCapture,
    from: usize,
    to: usize,
) -> opencv::error::Result<Vec<Mat>> {
    let mut img = Mat::default();
    for _ in 0..from {
        vc.read(&mut img)?;
    }
    assert!(to >= from);
    let mut frames = vec![];
    // TODO: iterator で書こうとすると Result が混線して微妙
    for _ in 0..(to - from) {
        let mut frame = Mat::default();
        let grabbed = vc.read(&mut frame)?;
        if grabbed {
            frames.push(frame);
        }
    }
    Ok(frames)
}

/// nth frame (0-indexed) を取り出す．
pub fn get_nth_frame(vc: &mut VideoCapture, n: usize) -> opencv::error::Result<Mat> {
    let mut img = Mat::default();
    for _ in 0..n {
        vc.read(&mut img)?;
    }
    let mut the_frame = Mat::default();
    let _ = vc.read(&mut the_frame)?;
    Ok(the_frame)
}

/// 飛び飛びのフレームも想定して，与えられた列のフレームを返す．
/// !! 結果はフレーム番号の昇順になる．
/// * 途中でフレームが空になっても許容することにするが，そこは飛ばす
/// * returns: [opencv::error::Result]<[Vec]<[Mat]>>
///   [VideoCaptureTrait::read](opencv::prelude::VideoCaptureTrait::read) が Err を返すときに同様にそれを返す
pub fn get_nth_frames(
    vc: &mut VideoCapture,
    ns: &[usize],
) -> opencv::error::Result<Vec<(usize, Mat)>> {
    let mut frames: Vec<usize> = ns.to_vec();
    frames.sort();
    let mut n = 0; // 今何フレーム目読んでるか
    let mut img = Mat::default();
    let mut result = vec![];
    for &next_target in &frames {
        // 次のところまで読み飛ばす
        while n < next_target {
            vc.grab()?;
            n += 1;
        }
        let read_st = vc.read(&mut img)?;
        n += 1;
        // 空じゃないフレームを読めてたら結果に追加
        if read_st {
            result.push((next_target, img.clone()));
        }
    }
    Ok(result)
}

#[derive(Debug)]
pub enum FrameSetting {
    Single(usize),
    Seq(usize, usize),
    Frames(Vec<usize>),
}

#[derive(Debug)]
pub struct GetNthFrame {
    pub file: String,
    pub frame_setting: FrameSetting,
    pub spawn_setting: SpawnSettings,
}

impl GetNthFrame {
    pub fn run(&self) {
        let (mut vc, c) = load_video(&self.file).unwrap();
        if c == 0 {
            panic!("GetNthFrame::run: loaded video with zero frame");
        }
        let frames_to_save: Vec<(usize, Mat)> = match &self.frame_setting {
            &FrameSetting::Single(n) => {
                let f = get_nth_frame(&mut vc, n).unwrap();
                vec![(n, f)]
            }
            &FrameSetting::Seq(from, to) => {
                let fs = get_frame_fromto(&mut vc, from, to).unwrap();
                fs.into_iter()
                    .enumerate()
                    .map(|(i, f)| (i + from, f))
                    .collect()
            }
            FrameSetting::Frames(ns) => get_nth_frames(&mut vc, &ns[..]).unwrap(),
        };
        save_mats_as(&self.spawn_setting, &frames_to_save);
    }
}
