use super::board::{self, BoardModel};
use super::piece::{self, PieceBoardPosition, PieceModel};

pub struct SolverSingleThreaded {
    pieces: [PieceModel; 8],
    board: BoardModel,
    solution_set: Vec<Vec<PieceBoardPosition>>,
}

impl SolverSingleThreaded {
    pub fn new(day: usize, month: usize) -> SolverSingleThreaded {
        SolverSingleThreaded {
            pieces: piece::create_piece_models(),
            board: BoardModel::new(day, month),
            solution_set: Vec::new(),
        }
    }

    /// Returns an immutable reference to pieces field.
    pub fn get_pieces(&self) -> &[PieceModel; 8] {
        &self.pieces
    }

    /// Returns an immutable reference to the solution_set field.
    pub fn get_solution_set(&self) -> &Vec<Vec<PieceBoardPosition>> {
        &self.solution_set
    }

    /// Returns a solution set for solver.
    ///
    /// Iterates through all possible combinations and appends valid
    ///  solutions to the `solution_set` vector.
    pub fn find_solution_set(&mut self, mut start_index: usize) {
        // Create memento to handle state
        let mut solver_history: Vec<usize> = Vec::new();

        loop {
            // Flag to determine when to go back to previous state
            let mut restore_last_state = true;

            // Get next available board position
            let board_position = board::next_board_position(self.board.get_board_layout());

            // Get next eligible piece to be placed
            'piece_loop: for index in start_index..self.get_pieces().len() {
                let piece = &mut self.pieces[index];

                if !piece.is_used() {
                    while !piece.is_exhausted() {
                        if self.board.is_piece_valid(board_position, piece) {
                            // Set flag to indicate piece is used
                            piece.set_used(true);

                            // Save current board state
                            self.board.generate_memento();

                            // Update board state
                            self.board.add_piece_to_board(piece);

                            // Save current state of solver
                            solver_history.push(index);

                            // Update loop flag
                            restore_last_state = false;

                            // Reset start_index
                            start_index = 0;

                            break 'piece_loop;
                        } else {
                            piece.next_unique_orientation();
                        }
                    }

                    // Orientations Exhausted - reset piece
                    piece.reset();
                }
            }

            // Check if board is complete. Add solution to solution set if it is complete.
            if board::is_board_complete(self.board.get_board_layout()) {
                // A solutions has been found. Record solution
                let mut solution: Vec<PieceBoardPosition> = Vec::new();
                for piece in &self.pieces {
                    solution.push(piece.get_piece_board_position());
                }

                // Append solution to solution set
                self.solution_set.push(solution);

                // Change flag to indicate to undo last move
                restore_last_state = true;
            }

            // If hasn't broken out, has exhausted all possibilities for current loop.
            // Return to previous solver state or break from loop if finished searching.
            if restore_last_state {
                match solver_history.pop() {
                    Some(x) => {
                        start_index = x;

                        // return to previous board position
                        self.board.restore_from_memento();

                        // Remove flag indicating piece is used
                        self.pieces[start_index].set_used(false);
                        self.pieces[start_index].set_board_position(None);

                        // Get next unique orientation of piece
                        self.pieces[start_index].next_unique_orientation();
                    }
                    // Completed Search, break from outer loop
                    None => break,
                }
            }
        }
    }

    pub fn remove_duplicates(&mut self) {
        self.solution_set.sort();
        self.solution_set.dedup();
    }
}
