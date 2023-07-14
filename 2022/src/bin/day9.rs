use input_extractor::get::{get_input, AoCDate};
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub struct Position {
    i: usize,
    j: usize,
}

impl Position {
    fn new(i: usize, j: usize) -> Self {
        Position { i, j }
    }
    fn move_left(&mut self) {
        self.i -= 1;
    }
    fn move_right(&mut self) {
        self.i += 1;
    }
    fn move_up(&mut self) {
        self.j += 1;
    }
    fn move_down(&mut self) {
        self.j -= 1;
    }
    fn follow_i(&mut self, i_pos: usize) {
        self.i = (self.i + i_pos) / 2;
    }
    fn follow_j(&mut self, j_pos: usize) {
        self.j = (self.j + j_pos) / 2;
    }
}

fn tail_diagonal_step(tail: &Position, head: &Position) -> Position {
    let mut new_position = *tail;
    if head.i > tail.i {
        new_position.move_right();
    } else {
        new_position.move_left();
    }
    if head.j > tail.j {
        new_position.move_up();
    } else {
        new_position.move_down();
    }
    new_position
}

fn tail_next_position(tail: &Position, head: &Position) -> Position {
    let mut new_position = *tail;
    if (head.i - 1..head.i + 2).contains(&tail.i) && (head.j - 1..head.j + 2).contains(&tail.j) {
        return new_position;
    }
    if tail.i == head.i && (tail.j as isize - head.j as isize).abs() == 2 {
        new_position.follow_j(head.j);
        return new_position;
    }
    if tail.j == head.j && (tail.i as isize - head.i as isize).abs() == 2 {
        new_position.follow_i(head.i);
        return new_position;
    }
    tail_diagonal_step(tail, head)
}

fn main() {
    let content = get_input(AoCDate { year: 2022, day: 9 });

    // this gives me a unique vector
    let mut grid: HashSet<(usize, usize)> = HashSet::new();

    let mut head = Position::new(500, 500);
    let mut tail = Position::new(500, 500);

    grid.insert((tail.i, tail.j));

    for line in content.lines() {
        let elements: Vec<_> = line.split_whitespace().collect();
        let moves = FromStr::from_str(elements[1]).unwrap();

        match elements[0] {
            "L" => {
                for _ in 0..moves {
                    head.move_left();
                    tail = tail_next_position(&tail, &head);
                    grid.insert((tail.i, tail.j));
                }
            }
            "R" => {
                for _ in 0..moves {
                    head.move_right();
                    tail = tail_next_position(&tail, &head);
                    grid.insert((tail.i, tail.j));
                }
            }
            "U" => {
                for _ in 0..moves {
                    head.move_up();
                    tail = tail_next_position(&tail, &head);
                    grid.insert((tail.i, tail.j));
                }
            }
            "D" => {
                for _ in 0..moves {
                    head.move_down();
                    tail = tail_next_position(&tail, &head);
                    grid.insert((tail.i, tail.j));
                }
            }
            _ => println!("This should never happen! {}", elements[0]),
        }
    }
    let cell_visited = grid.len();
    println!("Number of cells visited {:?}", cell_visited);
}
