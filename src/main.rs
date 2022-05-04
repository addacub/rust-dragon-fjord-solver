use std::time::SystemTime;

use dfsolver::puzzle::{piece::PieceBoardPosition, solver::SolverSingleThreaded};

fn main() {
    let day = 31;
    let month = 1;

    let start_time = SystemTime::now();
    let mut dragon = SolverSingleThreaded::new(day, month);
    dragon.find_solution_set(0);
    let end_time = SystemTime::now();
    println!(
        "Program took {:#?} seconds to execute.",
        end_time.duration_since(start_time)
    );
    println!(
        "{} solution(s) were found.",
        dragon.get_solution_set().len()
    );
    dragon.remove_duplicates();
    println!(
        "{} unique solution(s) were found.",
        dragon.get_solution_set().len()
    );
    print_solution(0, dragon.get_solution_set());
}

/// Print out the specified solution from the solution set
fn print_solution(index: usize, solution_set: &Vec<Vec<PieceBoardPosition>>) {
    println!(
        "Solution {} of {} is shown below:",
        index,
        solution_set.len()
    );
    let solution = &solution_set[index];
    for piece in solution {
        println!(
            "Place {} at position (row, col) = {:#?} with the following orientation:",
            piece.get_name(),
            piece.get_board_position()
        );
        println!("{}", piece.get_orienation());
        println!("\n");
    }
}
