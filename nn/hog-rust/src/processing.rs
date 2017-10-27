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
//         .map(|(i, x)| {

//         })
// }

pub fn gradient(left:u8, right:u8, top:u8, bottom:u8) -> GradientVector {
    let x:i16 = (right - left) as i16;
    let y:i16 = (top - bottom) as i16;
    GradientVector {
        data: vec![x, y],
        magnitude: ((x.pow(2) + y.pow(2)) as f64).sqrt(),
        angle: ((x/y) as f64).atan()
    }
}
