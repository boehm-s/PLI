use pixels::RgbaImage;

pub fn intensities(img:RgbaImage) -> Vec<Vec<f32>> {
    let mut lumas:Vec<Vec<f32>> = Vec::new();

    for y in 0..img.size.height as usize {
        lumas[y] = Vec::new();

        for x in 0..img.size.height as usize {
            let i = x * 4 + y * 4 * img.size.width as usize;
            let (r, g, b, a):(f32, f32, f32, f32) = (
                img.raw_data[i] as f32,
                img.raw_data[i + 1] as f32,
                img.raw_data[i + 2] as f32,
                img.raw_data[i + 3] as f32
            );
            let luma:f32 = if a == 0.0_f32 {
                1.0_f32
            } else { (r * 299.0/1000.0 + g * 587.0/1000.0 + b * 114.0/1000.0) / 255.0 };
            lumas[y][x] = luma;
        }
    }
    lumas
}
