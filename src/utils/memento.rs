use core::slice;

use super::array_2d::Array2D;


pub struct RecursiveHistory {
    history: Vec<Box<Memento>>,
}

impl RecursiveHistory {
    pub fn new() -> RecursiveHistory {
        RecursiveHistory { history: Vec::new() }
    }

    fn add_memento(&mut self, memento: Box<Memento>) {
        self.history.push(memento);
    }

    fn get_memento(&mut self) -> Box<Memento> {
        self.history.pop().unwrap()

    }
}


pub struct Memento {
    backup: Array2D,
}

impl Memento {
    /// Creates a `Memento` of the current state of the `BoardModel` it is called on.
    pub fn new(backup: Array2D) -> Memento {
        Memento { backup }
    }

    pub fn get_state(self) -> Array2D {
        self.backup
    }
}
