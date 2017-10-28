use matrix::Matrix;

#[derive(Debug)]
pub struct GradientVector {
    data: Vec<i16>,
    magnitude: f64,
    angle: f64
}

// pub fn grayscale_gradient(img: Matrix<u8>) -> Matrix<GradientVector> {
//     let new_data = img.data
//         .into_iter()
//         .enumerate()
//         .map(|(i, el)| {

//         })
// }

pub fn surroundings_pixels(img: Matrix<u8>, x:u8, y:u8) -> Vec<u8> {
    let mut left:u8 = 0;
    let mut right:u8 = 0;
    let mut top:u8 = 0;
    let mut bottom:u8 = 0;

    let pixel_pos = (img.cols * y as usize) + x as usize;

    if y != 0 { top = (pixel_pos - img.cols) as u8; }
    if x != 0 { left = (pixel_pos - 1) as u8; }
    if y != (img.rows - 1) as u8 { bottom = (pixel_pos + img.cols) as u8; }
    if x != (img.cols - 1) as u8 { right = (pixel_pos + 1) as u8 }

    vec![left, right, top, bottom]
}

pub fn gradient(left:u8, right:u8, top:u8, bottom:u8) -> GradientVector {
    let x:i16 = (right - left) as i16;
    let y:i16 = (top - bottom) as i16;
    GradientVector {
        data: vec![x, y],
        magnitude: ((x.pow(2) + y.pow(2)) as f64).sqrt(),
        angle: ((x/y) as f64).atan()
    }
}
