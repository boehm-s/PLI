use std::fmt;

#[derive(Debug)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>
}

impl <T> Matrix<T> {
    pub fn new(rows: usize, cols: usize, data: Vec<T>) -> Matrix<T> {
        assert!(cols * rows == data.len(), "Data does not match given dimensions.");

        Matrix {
            rows: rows,
            cols: cols,
            data: data
        }
    }
}

// Display matrix : need to be done correctly
impl fmt::Display for Matrix<Vec<u8>> {
    /// Formats the Matrix for display.
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut max_pixels_width = 0;
        for pixels in &self.data {
            let pixels_width = match f.precision() {
                Some(places) => format!("{:?}", pixels).len(),
                None => format!("{:?}", pixels).len(),
            };
            if pixels_width > max_pixels_width {
                max_pixels_width = pixels_width;
            }
        }
        let width = max_pixels_width;

        fn format_pixels(pixels: &Vec<u8>) -> String {
            let formatted_str:String = pixels
                .into_iter()
                .enumerate()
                .map(|(i, x)| {
                    match i {
                        0 => format!("[{:03},", x),
                        2 => format!("{:03}]", x),
                        _ => format!("{:03},", x)
                    }
                })
                .collect();

            formatted_str
        }

        fn write_row(f: &mut fmt::Formatter, row: &[Vec<u8>], left_delimiter: &str, right_delimiter: &str, width: usize) -> Result<(), fmt::Error> {
            try!(write!(f, "{}", left_delimiter));
            for (index, pixels) in row.iter().enumerate() {
                match f.precision() {
                    Some(places) => { try!(write!(f, "{}", format_pixels(pixels))); }
                    None => { try!(write!(f, "{}", format_pixels(pixels))); }
                }
                if index < row.len() - 1 { try!(write!(f, " ")); }
            }
            write!(f, "{}", right_delimiter)
        }

        match self.rows {
            1 => write_row(f, &self.data, "[", "]", width),
            _ => {
                try!(write_row(f, &self.data[0..self.cols], "⎡",  "⎤",  width));
                try!(f.write_str("\n"));
                for row_index in 1..self.rows - 1 {
                    try!(write_row(f, &self.data[row_index * self.cols..(row_index + 1) * self.cols], "⎢",  "⎥",  width));
                    try!(f.write_str("\n"));
                }
                write_row(f, &self.data[(self.rows - 1) * self.cols..self.rows * self.cols], "⎣",  "⎦",  width)
            }
        }

    }
}
