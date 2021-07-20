use std::collections::{HashSet};

pub fn day6() {

    let example_input = std::fs::read_to_string("inputs/d6.example").unwrap();
    let input = std::fs::read_to_string("inputs/d6").unwrap(); 

    // task 1 test case
    assert_eq!(11_usize, parse_input_task1(&example_input).iter().map(|set| set.len()).sum());
    
    // task 1 with actual input
    let sum_task1 : usize= parse_input_task1(&input)
        .iter()
        .map(|set| set.len())
        .sum(); // sum values

    println!("task1: sum: {}", sum_task1);

    // task 2 test case
    assert_eq!(6_usize, parse_input_task2(&example_input).iter().map(|set| set.len()).sum());

    // task 2 with actual input
    let sum_task2 : usize = parse_input_task2(&input)
        .iter()
        .map(|set| set.len())
        .sum();

    println!("task2: sum: {}", sum_task2);
}

fn parse_input_task1(input: &str) -> Vec<HashSet<char>> {
    let mut groups = Vec::new();
    
    let mut answer_set = HashSet::new();
    for line in input.lines() {
        if !line.is_empty() {
            for c in line.chars() {
                answer_set.insert(c);
            }
        } else {
            let mut a = answer_set.clone();
            a.shrink_to_fit(); // the hashset will not be mutated further so shrink the capacity before storing it in the vec might be beneficial for memory usage
            groups.push(a);
            answer_set.clear();
        }
    }
    // make sure answers in the end without a trailing empty line are not dropped
    if !answer_set.is_empty() {
        // the set could potentially be shrinked before insertion to decrease memory usage
        groups.push(answer_set);
    }
    groups
}


fn parse_input_task2(input: &str) -> Vec<HashSet<char>> {
    let mut groups = Vec::new();
    // stores sets of each individual respondent  of a group
    let mut respondents = Vec::new();
    for line in input.lines() {
        if !line.is_empty() {
            let mut answer_set = HashSet::new();
            for c in line.chars() {
                answer_set.insert(c);
            }
            respondents.push(answer_set);
        } else {
            // Create of the replies of the respondents
            let group_union = slice_of_sets_union(&respondents);
            groups.push(group_union);
            respondents.clear();
        }
    }
    if !respondents.is_empty() {
        groups.push(slice_of_sets_union(&respondents));
    }
    groups
}

/// Creates a union of all sets in a slice
fn slice_of_sets_union<T>(vec_of_sets: &[HashSet<T>]) -> HashSet<T>
    where T: Clone + Eq + std::hash::Hash {
    let union: HashSet<T> = match vec_of_sets.len() {
        0 => HashSet::new(),
        1 => vec_of_sets[0].clone(),
        _n => {
            vec_of_sets[0]
                .clone()
                .into_iter()
                .filter(|k| vec_of_sets[1..].iter().all(|s| s.contains(k)))
                .collect()
        }
    };
    union
}