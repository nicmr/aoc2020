use std::fs;

pub fn day5() {
    // establish test cases
    let example_pass_1 = "FBFBBFFRLR";
    let example_pass_2 = "BFFFBBFRRR";
    let example_pass_3 = "FFFBBBFRRR";
    let example_pass_4 = "BBFFBBFRLL";

    // row tests
    assert_eq!(Some(44), parse_row(example_pass_1));
    assert_eq!(Some(70), parse_row(example_pass_2));
    assert_eq!(Some(14), parse_row(example_pass_3));
    assert_eq!(Some(102), parse_row(example_pass_4));

    // column tests
    assert_eq!(Some(5), parse_column(example_pass_1));
    assert_eq!(Some(7), parse_column(example_pass_2));
    assert_eq!(Some(7), parse_column(example_pass_3));
    assert_eq!(Some(4), parse_column(example_pass_4));
    
    println!("All test examples passed");

    // run part 1 with real data
    let input = fs::read_to_string("inputs/d5").unwrap();

    println!("line count: {}", input.lines().count());

    let rows_columns: Vec<(u64, u64)> = input
        .lines()
        .map(|line| (parse_row(line).unwrap(),  parse_column(line).unwrap()))
        .collect();

    let mut row_ids: Vec<u64> = rows_columns.iter()
        .map(|(row, col)| row * 8 + col)
        .collect(); 

    println!("Max value of input is {}", row_ids.iter().max().unwrap()); // safe to unwrap, we know the iterator is not empty

    // part two
    // requirements:
    // there exists a seat with seat_id = my_seat_id -1
    // there exists a seat with seat_id = my_seat_id + 1
    // => there exist two seats x,y where x - y == 2 and my_seat_id will be in between
    // i.e. we are looking for a hole in a vec of consecutive ids

    // my seat is not at the very front or back, => there exists a seat with row = my_row - 1 && there exists a seat with row = my_row + 1

    // If we sort the vec anyway, we could rewrite part 1 to use the last index of the sorted vec instead of calling max.
    println!("row id count 1: {}", row_ids.len());
    
    row_ids.sort_unstable();

    println!("row id count 2: {}", row_ids.len());
    row_ids.dedup();

    println!("row id count 3: {}", row_ids.len());

    // Weird issue: we find no hole in the vector.
    // Another weird issue: we encounter lots of duplicate IDs in the input data.


    // for id in row_ids {
    //     println!("{}", id)
    // }

    // let mut last = 0;
    // for id in row_ids {
    //     if id-last != 1 {
    //         println!("Possible id matchs found between {} and {}. Differnce:: {}", id, last, id-last);
    //     }
    //     last = id;
    // }    

}
/// Parse the seat number in the range [0, 127] using binary space partitioning
/// Returns None when trying to parse an unsupported letter
pub fn parse_row(bsp_str: &str) -> Option<u64> {
    let init_size = 128;
    let mut target_row = 0;
    for (depth, char) in bsp_str[..6].chars().enumerate() {
        match char {
            'B' => {
                target_row += init_size / 2_u64.pow((depth+1)as u32);
            }
            'F' => {
            },
            _ => {return None},
        }
        // println!("row:{}", target_row);
    }
    Some(target_row)
}

fn parse_column(bsp_str: &str) -> Option<u64> {
    let init_size = 8;
    let mut target_col = 0;
    for (depth, char) in bsp_str[7..].chars().enumerate() {
        match char {
            'R' => {
                target_col += init_size / 2_u64.pow((depth+1) as u32);
            }
            'L' => {
            },
            _ => {return None},
        }
        // println!("col:{}", target_col);
    }
    Some(target_col)
}