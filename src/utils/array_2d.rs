use core::fmt;
use std::mem;

/// Creates an `Array2D` struct from a passed in array like object.
/// The array data is stored in a 1D `vec`, and the array shape is stored in a `Shape` struct.
///
/// # Arguments
///
/// * `array` - A list like object written as a set of nested arrays used to represent a 2D array.
///
/// # Examples
///
/// Creating a new `Array2D` struct
///
/// ```
/// # use dfsolver::{utils::array_2d::*, array2D};
/// let array: Array2D = array2D![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
///
/// let expected_result: Array2D = Array2D::new(Shape { rows: Some(3), cols: Some(3) }, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
/// assert_eq!(expected_result, array);
/// ```
#[macro_export]
macro_rules! array2D {
    ( $( $row_vector: tt ),* ) => {
        {
            use $crate::utils::array_2d::{ Array2D, Shape };
            let mut data = Vec::new();
            let mut shape = Shape { rows: None, cols: None };

            $(
                let new_row = vec!$row_vector;

                // Set the length of the row (i.e. number of columns) if none has been set.
                // If set, assert that rows all have the same legnth.
                if shape.cols == None {
                    shape.cols = Some(new_row.len());
                } else {
                    assert!(new_row.len() == shape.cols.unwrap(), "Supplied matrix had inconsistent row lengths")
                }

                // Counts the number of rows in the matrix
                if shape.rows == None {
                    shape.rows = Some(1);
                } else {
                    shape.rows = Some(shape.rows.unwrap() + 1);
                }

                data.append(&mut vec!$row_vector);
            )*

            Array2D::new(shape, data)
        }
    };
}

/// A struct which corresponds to the shape of a M x N array.
///
/// # Arguments
///
/// * `rows` - the number of rows in the matrix (M dimension).
/// * `cols` - the number of columns in the matrix (N dimension).
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Shape {
    pub rows: Option<usize>,
    pub cols: Option<usize>,
}

/// An enumeration of the axes which can be flipped along.
///
/// * Flipping along the X axes corresponds to vertically flipping the matrix.
/// * Flipping along the Y axes corresponds to horizontally flipping the matrix.
pub enum Axes {
    X,
    Y,
}

/// A struct which corresponds to a 2D array.
///
/// # Arguments
///
/// * `shape` - the shape of the M x N array where M is the number of rows, and N is the number of columns
/// * `data` - a 1D `Vec` which holds the data of the array
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Array2D {
    shape: Shape,
    data: Vec<usize>,
}

impl Array2D {
    pub fn new(shape: Shape, data: Vec<usize>) -> Array2D {
        Array2D { shape, data }
    }

    /// Returns the element at the specified index of the `Array2D` it is called on.
    /// 
    /// # Arguments
    /// * `row` - The row of the desired element.
    /// * `col` - The column of the desired element.
    /// 
    /// # Panics!
    /// Function will if attempting to index outside the bounds of the array.
    /// 
    /// # Examples
    /// Array indexing within the bounds of the array.
    /// 
    /// ```
    /// # use dfsolver::{utils::array_2d::*, array2D};
    /// let matrix: Array2D = array2D!([1, 2, 3], [4, 5, 6]);
    /// assert_eq!(matrix.get(0, 1), 2)
    /// ```
    /// 
    /// Array indexing outside the bounds of the array
    /// 
    /// ```should_panic
    /// # use dfsolver::{utils::array_2d::*, array2D};
    /// let matrix: Array2D = array2D!([1, 2, 3], [4, 5, 6]);
    /// matrix.get(1, 4);
    /// ```
    pub fn get(&self, row: usize, col: usize) -> usize {
        self.data[self.shape.cols.unwrap() * row + col]
    }

    /// Flips a `Array2D` it is called on along the axes specified. 
    /// Only the data field is mutated. The shape field is left untouched.
    ///
    /// # Arguments
    ///
    /// * `axes` - An enum value of either `Axes::X` or `Axes::Y`
    ///
    /// # Examples
    ///
    /// Flipping along the x axis (corresponds to flipping vertically).
    ///
    /// ```
    /// # use dfsolver::{utils::array_2d::*, array2D};
    /// let mut matrix: Array2D = array2D!([1, 2, 3], [4, 5, 6], [7, 8, 9]);
    /// matrix.flip(Axes::X);
    ///
    /// let expected_result: Array2D = array2D!([7, 8, 9], [4, 5, 6], [1, 2, 3]);
    /// assert_eq!(expected_result, matrix);
    /// ```
    ///
    /// Flipping along the y axis (corresponds to flipping horizontally).
    /// ```
    /// # use dfsolver::{utils::array_2d::*, array2D};
    /// let mut matrix: Array2D = array2D!([1, 2, 3], [4, 5, 6], [7, 8, 9]);
    /// matrix.flip(Axes::Y);
    ///
    /// let expected_result: Array2D = array2D!([3, 2, 1], [6, 5, 4], [9, 8, 7]);
    /// assert_eq!(expected_result, matrix);
    /// ```

    pub fn flip(&mut self, axes: Axes) {
        match axes {
            Axes::X => {
                for row_index in 0..self.shape.cols.unwrap() / 2 {
                    for col_index in 0..self.shape.cols.unwrap() {
                        self.data.swap(
                            row_index * self.shape.cols.unwrap() + col_index,
                            ((self.shape.rows.unwrap() - 1) - row_index) * self.shape.cols.unwrap()
                                + col_index,
                        );
                    }
                }
            }
            Axes::Y => {
                for row_index in 0..self.shape.rows.unwrap() {
                    let row_slice = &mut self.data[row_index * self.shape.cols.unwrap()
                        ..row_index * self.shape.cols.unwrap() + self.shape.cols.unwrap()];
                    row_slice.reverse();
                }
            }
        }
    }

    /// Transposes the `Array2D` it is called on.
    /// Both the data field and the shape field are mutated.
    /// 
    /// # Examples
    /// Transposing a 2 x 4 array
    /// 
    /// ```
    /// # use dfsolver::{utils::array_2d::*, array2D};
    /// let mut matrix: Array2D = array2D!([0, 1, 2, 3], [4, 5, 6, 7]);
    /// matrix.transpose();
    /// let expected_result: Array2D = array2D!([0, 4], [1, 5], [2, 6], [3, 7]);
    /// assert_eq!(expected_result, matrix);
    /// ```
    /// 
    /// Calling transpose on an array which has been tranposed.
    /// ```
    /// # use dfsolver::{utils::array_2d::*, array2D};
    /// let mut matrix: Array2D = array2D!([0, 1, 2, 3], [4, 5, 6, 7]);
    /// matrix.transpose();
    /// matrix.transpose();
    /// let expected_result: Array2D = array2D!([0, 1, 2, 3], [4, 5, 6, 7]);
    /// assert_eq!(expected_result, matrix);
    /// 
    pub fn transpose(&mut self) {
        // The product of M x N which gives the length of the 1D array which represents the data
        let mn = self.shape.rows.unwrap() * self.shape.cols.unwrap();
        let mut visisted: Vec<bool> = vec![false; mn];

        let mut cycle_start = 0;

        while cycle_start != mn {
            let mut old_index = cycle_start;

            if !visisted[cycle_start] {
                loop {
                    let new_index = if old_index == mn - 1 {
                        mn - 1
                    } else {
                        (self.shape.cols.unwrap() * old_index) % (mn - 1)
                    };

                    if new_index == cycle_start {
                        visisted[old_index] = true;
                        break;
                    }

                    self.data.swap(old_index, new_index);
                    visisted[old_index] = true;
                    old_index = new_index;
                }
            }
            cycle_start += 1;
        }

        mem::swap(&mut self.shape.cols, &mut self.shape.rows);
    }

    // pub fn rotate90(matrix: Array2D, k: usize ) -> Array2D {
    //     if k == 0 {
    //         return matrix;
    //     }

    //     if k == 2 {
    //         flip(flip(matrix, vertical), horizontal);
    //     }

    //     if k == 1 {
    //         return transpose(flip(m, horizontal))
    //     } else {
    //         // k == 3
    //         return flip(transpose(matrix), horizontal)
    //     }
    // }
}

impl fmt::Display for Array2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;

        for row_index in 0..self.shape.rows.unwrap() {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.
            if row_index != 0 {
                write!(f, ",\n ")?;
            }

            write!(f, "[")?;

            for col_index in 0..self.shape.cols.unwrap() {
                if col_index != 0 {
                    write!(f, ", ")?;
                }
                write!(
                    f,
                    "{}",
                    self.get(row_index, col_index)
                )?;
            }
            write!(f, "]")?;
        }
        return write!(f, "]");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flip_horizontally() {
        let mut matrix: Array2D = array2D!([1, 2, 3], [4, 5, 6], [7, 8, 9]);
        matrix.flip(Axes::Y);

        let expected_result: Array2D = array2D!([3, 2, 1], [6, 5, 4], [9, 8, 7]);
        assert_eq!(expected_result, matrix);
    }

    #[test]
    fn flip_vertically() {
        let mut matrix: Array2D = array2D!([1, 2, 3], [4, 5, 6], [7, 8, 9]);
        matrix.flip(Axes::X);

        let expected_result: Array2D = array2D!([7, 8, 9], [4, 5, 6], [1, 2, 3]);
        assert_eq!(expected_result, matrix);
    }

    #[test]
    fn test_transpose_square() {
        let mut matrix: Array2D = array2D!([1, 2, 3], [4, 5, 6], [7, 8, 9]);
        matrix.transpose();

        let expected_result: Array2D = array2D!([1, 4, 7], [2, 5, 8], [3, 6, 9]);
        assert_eq!(expected_result, matrix);
    }

    #[test]
    fn test_transpose_rectangle() {
        let mut matrix: Array2D = array2D!([0, 1, 2, 3], [4, 5, 6, 7]);
        matrix.transpose();

        let expected_result: Array2D = array2D!([0, 4], [1, 5], [2, 6], [3, 7]);
        assert_eq!(expected_result, matrix);
    }

    #[test]
    fn test_transpose_tranpose() {
        let mut matrix: Array2D = array2D!([0, 1, 2, 3], [4, 5, 6, 7]);
        matrix.transpose();
        matrix.transpose();

        let expected_result: Array2D = array2D!([0, 1, 2, 3], [4, 5, 6, 7]);
        assert_eq!(expected_result, matrix);
    }
}
