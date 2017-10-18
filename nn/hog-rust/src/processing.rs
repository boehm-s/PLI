use pixels::RgbaImage;

pub fn intensities(img:RgbaImage) -> Vec<Vec<u8>> {
    let mut lumas:Vec<Vec<u8>> = Vec::new();
    let y = 0;

    for y in 0..img.metadata.height {
        lumas[y] = Vec::new();

        for x in 0..img.metadata.height {
            let i = x * 4 + y * 4 * img.metadata.width;
            let (r, g, b, a) = (img.raw_data[i], img.raw_data[i + 1], img.raw_data[i + 2], img.raw_data[i + 3]);
            let luma = if a == 0 { 1 } else { (r * 299/1000 + g * 587/1000 + b * 114/1000) / 255 };
            lumas[y][x] = luma;
        }
    }
    lumas
}
