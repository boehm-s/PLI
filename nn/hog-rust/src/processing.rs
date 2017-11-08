use matrix::Matrix;

#[derive(Debug)]
pub struct GradientVector {
    data: Vec<i64>,
    magnitude: f64,
    angle: f64
}

pub fn grayscale_gradient(img: Matrix<u8>) -> Matrix<GradientVector> {
    let cols = img.cols;
    let rows = img.rows;
    let new_data = img.data
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, _el)| {
            let x:u8 = (i % cols) as u8;
            let y:u8 = (i / cols) as u8;
            let border_pixels = surroundings_pixels(&img, x, y);
            gradient(
                border_pixels[0],
                border_pixels[1],
                border_pixels[2],
                border_pixels[3]
            )
        }).collect();

    Matrix::new(rows, cols, new_data)
}

pub fn surroundings_pixels(img: &Matrix<u8>, x:u8, y:u8) -> Vec<u8> {
    let mut left:u8 = 0;
    let mut right:u8 = 0;
    let mut top:u8 = 0;
    let mut bottom:u8 = 0;

    let pixel_pos = (img.cols * y as usize) + x as usize;

    // can do shorter
    if y != 0 {
        top = (pixel_pos - img.cols) as u8;
        top = img.data[top as usize];
    }
    if x != 0 {
        left = (pixel_pos - 1) as u8;
        left = img.data[left as usize];
    }
    if y != (img.rows - 1) as u8 {
        bottom = (pixel_pos + img.cols) as u8;
        bottom = img.data[bottom as usize];
    }
    if x != (img.cols - 1) as u8 {
        right = (pixel_pos + 1) as u8;
        right = img.data[right as usize];
    }

    vec![left, right, top, bottom]
}

pub fn gradient(left:u8, right:u8, top:u8, bottom:u8) -> GradientVector {
    let x = (right as i64 - left as i64);
    let y = (top as i64 - bottom as i64);
    GradientVector {
        data: vec![x, y],
        magnitude: ((x.pow(2) + y.pow(2)) as f64).sqrt(),
        angle: match y {
            0 => 0 as f64,
            _ => ((x/y) as f64).atan()
        }
    }
}
