use std::{fs, str};

pub fn day3 () {
    let example_input = fs::read_to_string("inputs/d3.example").unwrap();
    let input = fs::read_to_string("inputs/d3").unwrap();

    let example_lines: Vec<&str> = example_input
        .lines()
        .collect();

    let lines : Vec<&str> = input
        .lines()
        .collect();

    // part 1
    let trees_p1 = calculate_encountered_trees(&lines, 3, 1);
    println!("Part 1: {} trees encountered.", trees_p1);

    // part 2
    println!("Part 2:");
    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    // Ensure the algorithm works by testing it on the example values
    let product_of_all_routes_example : usize = slopes.iter()
        .map(|(right, down)| calculate_encountered_trees(&example_lines, *right, *down))
        .map(|tree_encounters| {println!("{} trees encountered", tree_encounters); tree_encounters})
        .product();
    println!("All example tree encounters multiplied return: {}", product_of_all_routes_example);

    // Now apply to the real input
    let product_of_all_routes: usize = slopes.iter()
        .map(|(right, down)| calculate_encountered_trees(&lines, *right, *down))
        .map(|tree_encounters| {println!("{} trees encountered", tree_encounters); tree_encounters})
        .product();
    println!("All tree encounters multiplied returns: {}", product_of_all_routes);
}

fn calculate_encountered_trees(lines: &[&str], step_right: usize, step_down: usize) -> usize {
    let single_line_len = lines[0].chars().count();

    let mut trees = 0;
    let mut pos_down = 0;
    let mut pos_right = 0;
    while pos_down < lines.len() {
        let opt_symbol = lines[pos_down].chars().nth(pos_right % single_line_len);
        if let Some(symbol) = opt_symbol {
            if symbol == '#' {
                trees += 1;
            }
        } else {
            println!("Failed to access symbol for line {}", pos_down);
        }
        pos_down += step_down;
        pos_right += step_right;
    }

    trees
}