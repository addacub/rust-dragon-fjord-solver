use std::ops::Range;

use crate::{
    array2D,
    utils::array_2d::{Array2D, Axes, Shape},
};

use crate::utils::memento::*;

use super::piece::PieceModel;

/// Creates an empty calendar (i.e. board with no puzzles placed and no date selected)
fn create_empty_calendar() -> Array2D {
    array2D!(
        [0, 0, 0, 0, 0, 0, 1],
        [0, 0, 0, 0, 0, 0, 1],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 1, 1, 1]
    )
}

pub struct BoardModel {
    day: usize,
    month: usize,
    board_layout: Array2D,
}

impl BoardModel {
    pub fn new(day: usize, month: usize) -> BoardModel {
        BoardModel {
            day,
            month,
            board_layout: initialise_calendar_layout(day, month, create_empty_calendar()),
        }
    }

    /// Checks if piece being placed in its current orientation at the board position is valid.
    ///
    /// # Arguments
    /// * `board_pos` - A tuple of the row and column position in which to place the puzzle piece
    /// * `piece_model` - The model of the puzzle piece in the current orientation to be placed onto the board.
    pub fn is_piece_valid(&self, (row, col): (usize, usize), piece_model: PieceModel) -> bool {
        // Check if translated board position (to take into account for spaces in puzzle piece)
        // is within bounds of the board.
        if piece_model.translation_count() > col {
            return false;
        }

        // Check if puzzle piece is within bounds of the board if placed.
        if row + piece_model.current_orientation().shape().rows - 1
            > self.board_layout.shape().rows - 1
            || col + piece_model.current_orientation().shape().cols > self.board_layout.shape().cols
        {
            return false;
        }

        // Check if piece will overlap with an existing piece
        let new_board_layout =
            self.board_layout.clone() + place_piece_on_board((row, col), &piece_model);
        if new_board_layout.data().contains(&2) {
            return false;
        }

        // Check if piece will leave any holes
        if is_unreachable_holes(new_board_layout) {
            return false;
        }

        // Valid move
        true
    }

    fn generate_memento(&self) -> Box<Memento> {
        Box::new(Memento::new(self.board_layout.clone()))
    }

    fn restore_from_memento(&mut self, memento: Memento) {
        self.board_layout = memento.get_state();
    }
}

/// Initialises the calender
fn initialise_calendar_layout(day: usize, month: usize, mut empty_layout: Array2D) -> Array2D {
    // Set day
    let (row, col) = get_calendar_position(day, 2, 6, 7);
    empty_layout.set((row, col), 1);

    // Set month
    let (row, col) = get_calendar_position(month, 0, 5, 6);
    empty_layout.set((row, col), 1);

    empty_layout
}

/// Returns the calendar position (row and column) for a calendar entry (either a day or a month).
///
/// # Arguments
/// `calendar_entry` - The day or month that the calendar position is wanted for.
/// `start_row` - The start row of days or months on the calendar.
/// `end-col` - The last valid column. Note, the length of a month row is shorter due to the board layout
/// `divisor` - The length of the rows.
fn get_calendar_position(
    calendar_entry: usize,
    start_row: usize,
    end_col: usize,
    divisor: usize,
) -> (usize, usize) {
    let quotient = calendar_entry / divisor;
    let remainder = calendar_entry % divisor;

    let (row, col) = if remainder == 0 {
        (start_row + quotient - 1, end_col)
    } else {
        (start_row + quotient, remainder - 1)
    };

    (row, col)
}

/// Returns the next empty board position to place a puzzle piece on.
pub fn next_board_position(board_layout: Array2D) -> (usize, usize) {
    for (index, &item) in board_layout.data().iter().enumerate() {
        if item == 0 {
            let (row, col) = (
                index / board_layout.shape().cols,
                index % board_layout.shape().cols,
            );
            return (row, col);
        }
    }

    panic!("Unable to find an empty board position.");
}

/// Checks if board is complete.
/// * If complete, the board layout should contain only values of 1.
/// * An incomplete board will contain values of 0.
pub fn is_board_complete(board_layout: &Array2D) -> bool {
    if board_layout.data().contains(&0) {
        return false;
    } else {
        return true;
    }
}

/// Places a puzzle piece in its current orientation onto a empty board at the position specified.
/// * Piece starting position (0, 0) is placed onto the board at the given position
///
/// # Arguments
/// * `(row, col)` - A tuple corresponding to the row and column of the board at which to place the puzzle piece
/// * `piece_model` - The puzzle piece in its current orientation to be placed onto an empty board.
///
/// # Panics!
/// If the specified board position results in the puzzle piece going outside of the board's bounds.
pub fn place_piece_on_board((row, col): (usize, usize), piece_model: &PieceModel) -> Array2D {
    // Check matrix is within bounds of the board for given board position
    if row + piece_model.current_orientation().shape().rows - 1 > 6
        || col + piece_model.current_orientation().shape().cols - 1 > 6
    {
        panic!("Attempted to place a piece on the board at a position that would cause the piece to go outside the bounds of the board.");
    }

    // Create an empty board
    let mut piece_on_board = Array2D::new(Shape { rows: 7, cols: 7 }, vec![0; 7 * 7]);

    for row_piece in 0..piece_model.current_orientation().shape().rows {
        for col_piece in 0..piece_model.current_orientation().shape().cols {
            piece_on_board.set(
                (row + row_piece, col + col_piece),
                piece_model.current_orientation().get(row_piece, col_piece),
            )
        }
    }

    piece_on_board
}

/// Determines if current layout contains any unreachable holes.
/// * A unreachable hole cannot be filled by a puzzle piece and indicates a dead solution branch.
fn is_unreachable_holes(board_layout: Array2D) -> bool {
    // Test if there are any holes first
    if is_board_complete(&board_layout) {
        // Board is complete and piece is valid.
        return false;
    }

    let (row, col) = next_board_position(board_layout);

    return true;
}

/// Returns a matrix of adjacent neighbours at the specified board position.
fn get_neighbours((mut row, mut col): (usize, usize), mut board_layout: Array2D) -> Array2D {
    // Check if board position is on top or bottom of board
    if row == 0 || row == board_layout.shape().rows - 1 {
        let mut extra_row = Array2D::new(
            Shape {
                rows: 1,
                cols: board_layout.shape().cols,
            },
            vec![0; board_layout.shape().cols],
        );

        // Check if board position is on top row
        if row == 0 {
            extra_row.append_array(board_layout, Axes::X);
            board_layout = extra_row;
            row += 1;
        } else {
            board_layout.append_array(extra_row, Axes::X)
        }
    }

    // Check if board position is on left or right side of board
    if col == 0 || col == board_layout.shape().cols - 1 {
        let mut extra_col = Array2D::new(
            Shape {
                rows: board_layout.shape().rows,
                cols: 1,
            },
            vec![0; board_layout.shape().rows],
        );

        // Check if board position is on left side
        if col == 0 {
            extra_col.append_array(board_layout, Axes::Y);
            board_layout = extra_col;
            col += 1;
        } else {
            board_layout.append_array(extra_col, Axes::Y);
        }
    }

    let mut neighbours: Vec<u8> = Vec::new();
    for row_offset in (Range { start: -1, end: 2 }) {
        // Convert types to be able to add negative offsets
        let (row, col): (i8, i8) = (row.try_into().unwrap(), col.try_into().unwrap());

        let first_index = (row + row_offset) * board_layout.shape().cols as i8 + (col - 1);
        let second_index = (row + (row_offset)) * board_layout.shape().cols as i8 + (col + 1);

        // Convert back to usize to index
        let first_index: usize = first_index.try_into().unwrap();
        let second_index: usize = second_index.try_into().unwrap();

        neighbours.append(&mut board_layout.data()[first_index..second_index + 1].to_vec());
    }

    Array2D::new(Shape { rows: 3, cols: 3 }, neighbours)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_calendar_position_month() {
        // Arrange & Act
        let position = get_calendar_position(5, 0, 5, 6);

        // Assert
        assert_eq!((0, 4), position);
    }

    #[test]
    fn get_calendar_position_day() {
        // Arrange & Act
        let position = get_calendar_position(21, 2, 6, 7);

        // Assert
        assert_eq!((4, 6), position);
    }

    #[test]
    fn get_next_board_position() {
        // Arrange
        let test_board_layout = array2D!(
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 1, 1, 1]
        );

        // Act
        let (row, col) = next_board_position(test_board_layout);

        // Assert
        assert_eq!((3, 0), (row, col));
    }

    #[test]
    #[should_panic]
    fn get_next_board_position_panic() {
        // Arrange
        let test_board_layout = array2D!(
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1]
        );

        // Act & Assert
        let (_, _) = next_board_position(test_board_layout);
    }

    #[test]
    fn test_incomplete_board() {
        // Arrange
        let test_board_layout = array2D!(
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 1, 1, 1]
        );

        // Act
        let is_board_complete = is_board_complete(&test_board_layout);

        // Assert
        assert_eq!(false, is_board_complete);
    }

    #[test]
    fn test_complete_board() {
        // Arrange
        let test_board_layout = array2D!(
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1]
        );

        // Act
        let is_board_complete = is_board_complete(&test_board_layout);

        // Assert
        assert_eq!(true, is_board_complete);
    }

    #[test]
    #[rustfmt::skip::macros(array2D)]
    fn test_add_piece_to_board() {
        // Arrange
        let piece = PieceModel::new(
            "2x4 Zig Zag".to_string(),
            array2D!(
                [0, 0, 1, 1],
                [1, 1, 1, 0]
            ),
            3,
            true,
        );

        let board_position = (2, 3);
        let expected_result = array2D!(
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 1, 1],
            [0, 0, 0, 1, 1, 1, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0]
        );

        // Act
        let piece_on_board = place_piece_on_board(board_position, &piece);

        // Assert
        assert_eq!(expected_result, piece_on_board);
    }

    #[test]
    #[should_panic]
    fn test_add_piece_to_board_invalid() {
        // Arrange
        let piece = PieceModel::new(
            "2x4 Zig Zag".to_string(),
            array2D!([0, 0, 1, 1], [1, 1, 1, 0]),
            3,
            true,
        );

        let board_position = (2, 4);

        // Act & Assert
        let _ = place_piece_on_board(board_position, &piece);
    }

    #[test]
    #[rustfmt::skip::macros(array2D)]
    fn test_get_neighbours_centre() {
        // Arrange
        let board_position = (3, 2);
        let board_layout = array2D!(
            [0, 1, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 1, 1, 0, 0, 1, 1],
            [0, 0, 5, 1, 1, 1, 0],
            [0, 0, 1, 0, 0, 0, 0],
            [0, 1, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0]
        );
        let expected_result: Array2D = array2D!(
            [1, 1, 0],
            [0, 5, 1],
            [0, 1, 0]
        );

        // Act
        let neighbours = get_neighbours(board_position, board_layout);

        // Assert
        assert_eq!(expected_result, neighbours);
    }

    #[test]
    #[rustfmt::skip::macros(array2D)]
    fn test_get_neighbours_top_LHCorner() {
        // Arrange
        let board_position = (0, 0);
        let board_layout = array2D!(
            [5, 1, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 1, 1, 0, 0, 1, 1],
            [0, 0, 0, 1, 1, 1, 0],
            [0, 0, 1, 0, 0, 0, 0],
            [0, 1, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0]
        );
        let expected_result: Array2D = array2D!(
            [0, 0, 0],
            [0, 5, 1],
            [0, 0, 0]
        );

        // Act
        let neighbours = get_neighbours(board_position, board_layout);

        // Assert
        assert_eq!(expected_result, neighbours);
    }

    #[test]
    #[rustfmt::skip::macros(array2D)]
    fn test_get_neighbours_bottom_RHCorner() {
        // Arrange
        let board_position = (6, 6);
        let board_layout = array2D!(
            [5, 1, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 1, 1, 0, 0, 1, 1],
            [0, 0, 0, 1, 1, 1, 0],
            [0, 0, 1, 0, 0, 0, 0],
            [0, 1, 1, 0, 0, 2, 1],
            [0, 0, 0, 0, 0, 1, 5]
        );
        let expected_result: Array2D = array2D!(
            [2, 1, 0],
            [1, 5, 0],
            [0, 0, 0]
        );

        // Act
        let neighbours = get_neighbours(board_position, board_layout);

        // Assert
        assert_eq!(expected_result, neighbours);
    }
}
