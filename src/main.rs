use dfsolver::{
    array2D,
    utils::array_2d::{self, Axes},
};

fn main() {
    let mut matrix: array_2d::Array2D = array2D!([1, 2, 3], [4, 5, 6], [7, 8, 9]);

    println!("{}\n", matrix);
    matrix.flip(Axes::X);
    println!("{}\n", matrix);

    let mut matrix: array_2d::Array2D = array2D!([1, 2, 3], [4, 5, 6], [7, 8, 9]);
    matrix.flip(Axes::Y);
    println!("{}\n", matrix);

    let mut matrix: array_2d::Array2D = array2D!([1, 2, 3], [4, 5, 6], [7, 8, 9]);
    matrix.transpose();
    println!("{}\n", matrix);

    let mut matrix: array_2d::Array2D = array2D!([1, 2, 3], [4, 5, 6], [7, 8, 9]);
    matrix.transpose();
    matrix.transpose();
    println!("{}\n", matrix);
}
