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

pub fn print_matrix<T>(matrix: Matrix<T>, format_type:&Fn(&[T]) -> String) {
    let m = matrix.cols - 1;

    matrix.data
        .chunks(matrix.cols)
        .enumerate()
        .for_each(|(i, x)| {
            let _str = format_type(x);

            match i {
                0 => print_matrix_row(_str, 0),
                _ if i == m => print_matrix_row(_str, 2),
                _ => print_matrix_row(_str, 1)
            }
        });

}



fn format_u8(f:&[u8]) -> String {
    let _str = f
        .to_vec()
        .into_iter()
        .enumerate()
        .map(|(i, x)| match i {
            _ if i == f.to_vec().len() - 1 => format!("{}", x),
            _ => format!("{}, ", x)
        })
        .collect::<String>();

    format!("{}", _str)
}

pub fn print_matrix_int(matrix: Matrix<u8>) {
    print_matrix(matrix, &format_u8);
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
