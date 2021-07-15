use std::fs;

mod day2;

fn main() {
    day2::day2();
}

fn day1(){
    let input = fs::read_to_string("inputs/d1").unwrap();

    let values : Vec<i64> =
        input
            .split('\n')
            .filter_map(|s| s.parse::<i64>().ok())
            .collect();

    // PART 1
    // inefficient implementation: compare each integer with each other integer if they add up to 2020
    // Complexity: O(n^2)
    'outer: for (i1, value1) in values.iter().enumerate() {
        for (i2, value2) in values.iter().enumerate() {
            // check for identity and if they add up to 2020
            if i1 != i2 && value1 + value2 == 2020 {
                let multiplied = value1 * value2;
                println!("Solution found: {} and {}. Multiplied: {}", value1, value2, multiplied);
                // assume there is only one match. break the outer loop immediately.
                break 'outer;
            }
        }
    }

    // efficient idea:
    // sort the vec: O(n log(n))
    // move from the lowest value and the highest value to middle(?)
    // look for target/2
    // only compare the values below with those above
    // does this work recursively? for something like a mergesort
    // ex. adding up to 20:
    // 3, 5, 7, 10, 13, 16, 19
    // sort in place
    // values.sort_unstable();
    
    // PART 2
    // for the first attempt, reuse the inefficient implementation.
    // This time, the complexity becomes O(n^3).
    // Overall, the complexity is O(n^k) where k is the amount of numbers that have to be added.
    // This does not scale well.

    'outer_p2: for (i1, value1) in values.iter().enumerate() {
        for (i2, value2) in values.iter().enumerate() {
            for (i3, value3) in values.iter().enumerate() {
                // check for identity and if they add up to 2020
                // the identity check for i2 != i3 is implied by the other two checks
                if i1 != i2 && i1 != i3 && value1 + value2 + value3 == 2020 {
                    let multiplied = value1 * value2 * value3;
                    println!("Solution found: {} and {} and {}. Multiplied: {}", value1, value2, value3, multiplied);
                    // assume there is only one match. break the outer loop immediately.
                    break 'outer_p2;
                }
            }
        }
    }
}