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

    pub fn get(&mut self, row: usize, col: usize) -> &T {
        let n = row * self.cols + col;
        &self.data[n]
    }
}

// index starts at 0

//   need to implement separator and space optional arguments
//   to customize the matrix display in a more efficient way

pub fn print_matrix<T>(matrix: Matrix<T>, format_type:&Fn(&T) -> String) {
    let m = matrix.cols - 1;

    matrix.data
        .chunks(matrix.cols)
        .enumerate()
        .for_each(|(i, x)| {
            let _str = format_ref_array(x, format_type);

            match i {
                0 => print_matrix_row(_str, 0),
                _ if i == m => print_matrix_row(_str, 2),
                _ => print_matrix_row(_str, 1)
            }
        });
}

fn format_ref_array<T>(ref_array:&[T], format_type:&Fn(&T) -> String) -> String {
    let mut _str = String::from("");
    let mut i = 0;

    for x in ref_array {
        if i == ref_array.len() - 1 {
            _str = format!("{}{}", _str, format_type(x));
        } else {
            _str = format!("{}{}, ", _str, format_type(x));
        }

        i = i + 1;
    }

    _str
}

fn format_u8(t:&u8) -> String {
    format!("{}", t)
}

fn format_vecu8(t:&Vec<u8>) -> String {
    t
        .into_iter()
        .enumerate()
        .map(|(i, x)| match i {
            0 => format!("({:03}, ", x),
            _ if i == t.len() - 1 => format!("{:03})", x),
            _  => format!("{:03}, ", x)
        })
        .collect::<String>()
}


pub fn print_matrix_int(matrix: Matrix<u8>) {
    print_matrix(matrix, &format_u8);
}
pub fn print_matrix_vec(matrix: Matrix<Vec<u8>>) {
    print_matrix(matrix, &format_vecu8);
}

pub fn print_matrix_row(row:String, pos:i32) {
    match pos {
        0 => println!("⎡ {} ⎤", row),
        1 => println!("⎢ {} ⎥", row),
        2 => println!("⎣ {} ⎦", row),
        _ => println!("⎢ {} ⎥", row)
    }
}

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
