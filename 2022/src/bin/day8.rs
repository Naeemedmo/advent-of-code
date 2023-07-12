use input_extractor::get::{get_input, AoCDate};
use std::cmp::max;

fn main() {
    let content = get_input(AoCDate { year: 2022, day: 8 });
    let columns = content.lines().next().unwrap().chars().collect::<Vec<_>>().len();

    let mut tree_matrix = Vec::new();
    let mut rows: usize = 0;
    for line in content.lines() {
        let mut row_array = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        tree_matrix.append(&mut row_array);
        rows += 1;
    }

    // we do it once as the number of columns is constant
    let mut n_visible = 0;

    let mut scenic_score = 0;

    for row  in 1..(rows - 1) {
        for column  in 1..(columns - 1) {

            let mut left_score = 0;
            let mut right_score = 0;
            let mut up_score = 0;
            let mut down_score = 0;

            let mut is_visible_left = true;
            let mut is_visible_right = true;
            let mut is_visible_up = true;
            let mut is_visible_down = true;
            let tree = &tree_matrix[row * columns + column];
            for i in (0..row).rev() {
                left_score +=1;
                if tree <= &tree_matrix[i * columns + column] {
                    is_visible_left = false;
                    break;
                }
            }
            for i in row + 1..rows {
                right_score +=1;
                if tree <= &tree_matrix[i * columns + column] {
                    is_visible_right = false;
                    break;
                }
            }
            for j in (0..column).rev() {
                up_score +=1;
                if tree <= &tree_matrix[row * columns + j] {
                    is_visible_up = false;
                    break;
                }
            }
            for j in column + 1..columns {
                down_score +=1;
                if tree <= &tree_matrix[row * columns + j] {
                    is_visible_down = false;
                    break;
                }
            }
            if is_visible_left || is_visible_right || is_visible_up || is_visible_down {n_visible += 1;}
            let tree_scenic_score: u32 = left_score * right_score * up_score * down_score;
            scenic_score = max(scenic_score, tree_scenic_score);
        }
    }
    n_visible += 2 * rows  + 2 * columns - 4;
    println!("Solution part 1 {:?}", n_visible);
    println!("Solution part 2 {:?}", scenic_score);
}
