use crate::puzzle::board;

/// Puzzle piece that can be placed on board
pub struct Piece {
    // Arguments
    name: String,
    // initial_orientation: None,
    // current_orientation: ArrayStorage,
    board_position: board::BoardPosition,

    // Restrictions
    max_rotations: u32,
    is_flippable: bool,

    // Flags
    rotation_count: u32,
    translation_count: u32,
    has_flipped: bool,
    orientation_exhausted: bool,
    translation_exhausted: bool,
    is_used: bool,
}

impl Piece {
    /// Rotate the piece by 90 degrees
    pub fn rotate(&self) {
        // ArrayStorage.rotate();
        // self.rotation_count = self.rotation_count + 1;
    }
}
