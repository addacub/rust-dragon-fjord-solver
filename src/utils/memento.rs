use core::slice;

use super::array_2d::Array2D;

pub struct RecursiveHistory {
    history: Vec<Box<Memento>>,
}

impl RecursiveHistory {
    pub fn new() -> RecursiveHistory {
        RecursiveHistory {
            history: Vec::new(),
        }
    }

    fn add_memento(&mut self, memento: Box<Memento>) {
        self.history.push(memento);
    }

    fn get_memento(&mut self) -> Box<Memento> {
        self.history.pop().unwrap()
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::array2D;

    #[test]
    fn get_memento_state() {
        // Arrange
        let memento = Memento {
            backup: array2D!([1, 2, 3], [1, 2, 3], [1, 2, 3]),
        };
        let expected_result = array2D!([1, 2, 3], [1, 2, 3], [1, 2, 3]);

        // Act
        let backup = memento.get_state();

        // Assert
        assert_eq!(expected_result, backup);
    }

    #[test]
    fn test_add_memento() {
        // Arrange
        let memento = Box::new(Memento {
            backup: array2D!([1, 2, 3], [1, 2, 3], [1, 2, 3]),
        });
        let mut recursive_history = RecursiveHistory::new();
        let expected_result = Box::new(Memento {
            backup: array2D!([1, 2, 3], [1, 2, 3], [1, 2, 3]),
        });

        // Act
        recursive_history.add_memento(memento);

        // Assert
        assert_eq!(expected_result, *recursive_history.history.get(0).unwrap());
        assert_eq!(1, recursive_history.history.len());
    }

    #[test]
    fn test_get_memento() {
        // Arrange
        let memento = Box::new(Memento {
            backup: array2D!([1, 2, 3], [1, 2, 3], [1, 2, 3]),
        });
        let mut recursive_history = RecursiveHistory::new();
        let expected_result = Box::new(Memento {
            backup: array2D!([1, 2, 3], [1, 2, 3], [1, 2, 3]),
        });

        // Act
        recursive_history.add_memento(memento);
        let backup = recursive_history.get_memento();

        // Assert
        assert_eq!(expected_result, backup);
        assert_eq!(0, recursive_history.history.len())
    }
}
