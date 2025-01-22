use opencv::prelude::*;
use opencv::videoio::VideoCapture;

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
