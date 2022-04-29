use crate::{
    array2D,
    utils::array_2d::{Array2D, Shape},
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

/// Returns an array of 0's with the same dimension as the board.
/// TODO - May not be needed
fn create_empty_board() -> Array2D {
    Array2D::new(Shape { rows: 7, cols: 7 }, vec![0; 7 * 7])
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

    /// Returns the next empty board position to place a puzzle piece on
    pub fn next_board_position(&self) -> (usize, usize) {
        for (index, &item) in self.board_layout.data().iter().enumerate() {
            if item == 0 {
                let (row, col) = (
                    index / self.board_layout.shape().cols,
                    index % self.board_layout.shape().cols,
                );
                return (row, col);
            }
        }

        panic!("Unable to find an empty board position.");
    }

    /// Checks if board is complete.
    /// If complete, the board layout should contain only values of 1.
    /// An incomplete board will contain values of 0.
    pub fn is_board_complete(&self) -> bool {
        if self.board_layout.data().contains(&0) {
            return false;
        } else {
            return true;
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
        if row + piece_model.current_orientation().shape().rows - 1 > self.board_layout.shape().rows - 1
            || col + piece_model.current_orientation().shape().cols > self.board_layout.shape().cols
        {
            return false;
        }

        // Check if piece will overlap with an existing piece


        // Check if piece will leave any holes
        

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
    empty_layout.set(row, col, 1);

    // Set month
    let (row, col) = get_calendar_position(month, 0, 5, 6);
    empty_layout.set(row, col, 1);

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
        let test_board: BoardModel = BoardModel {
            day: 21,
            month: 5,
            board_layout: test_board_layout,
        };

        // Act
        let (row, col) = test_board.next_board_position();

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
        let test_board: BoardModel = BoardModel {
            day: 21,
            month: 5,
            board_layout: test_board_layout,
        };

        // Act & Assert
        let (_, _) = test_board.next_board_position();
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
        let test_board: BoardModel = BoardModel {
            day: 21,
            month: 5,
            board_layout: test_board_layout,
        };

        // Act
        let is_board_complete = test_board.is_board_complete();

        // Assert
        assert_eq!(false, is_board_complete);
    }

    #[test]
    #[should_panic]
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
        let test_board: BoardModel = BoardModel {
            day: 21,
            month: 5,
            board_layout: test_board_layout,
        };

        // Act
        let is_board_complete = test_board.is_board_complete();

        // Assert
        assert_eq!(true, is_board_complete);
    }
}
