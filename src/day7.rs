use std::{collections::HashMap};

use regex::Regex;

pub fn day7 () {
    let example_input = std::fs::read_to_string("inputs/d7.example").unwrap();
    let input = std::fs::read_to_string("inputs/d7").unwrap();

    // test input
    let bag_map_example = parse_bag_map(&example_input);

    let shiny_gold_count_example = bag_map_example.iter()
        .map(|(name, _contents)| contains(&bag_map_example, "shiny gold", name))
        .filter(|a| *a)
        .count();

    // test case
    assert_eq!(4, shiny_gold_count_example);

    // real data
    let bag_map =  parse_bag_map(&input);

    let shiny_gold_count = bag_map.iter()
        .map(|(name, _contents)| contains(&bag_map, "shiny gold", name))
        .filter(|a| *a)
        .count();
    
    println!("shiny gold count: {}", shiny_gold_count);

    let bags_inside_shiny_golds = calc_contents(&bag_map, "shiny gold");
    println!("bags inside shiny gold: {}", bags_inside_shiny_golds);
}

/// Parses the input into a data structure representing each bag and its contents
fn parse_bag_map (input: &str) -> HashMap<String, Vec<(u64, String)>> {
    // to avoid excessively complicated regex, we'll split the regex into two parts
    // 1. the bag identifier regex
    let bag_re = Regex::new(r"([\w ]+) bags contain (.+)").unwrap();

    // 2. the regex for the contents of the bags
    let contents_re = Regex::new(r"(\d)+\s+([\w ]+) bags?").unwrap();

    let bag_map : HashMap<String, Vec<(u64, String)>> =  input.lines()
        // capture bag identifier and contents
        .map(|line| {
            let captures= bag_re.captures(line).unwrap();
            (captures[1].to_owned(), captures[2].to_owned())
        })
        .fold(HashMap::new(), |mut hm, (name, content_string)| {
            
            let mut content_vec : Vec<(u64, String)> = Vec::new();

            for captures in contents_re.captures_iter(&content_string) {
                content_vec.push((captures[1].parse::<u64>().unwrap(), captures[2].to_owned()));
            }
            hm.insert(name, content_vec);
            hm
        });
    bag_map
}

/// Attempts to find a direct or indirect possibility of containing a bag with the provided name
/// Returns whether such a possibility was found or not
/// A more sophisticated, efficent solution would cache if a bag was already evaluated.
fn contains(bag_map: &HashMap<String, Vec<(u64, String)>>, search_name: &str, search_in: &str) -> bool {
    let (_bag, contents) = bag_map.get_key_value(search_in).unwrap();
    for (_count, contained_name) in contents {
        if contained_name == search_name {
            return true;
        } else {
            if contains(bag_map, search_name, contained_name) {
                return true;
            }
        }
    }
    false
}

fn calc_contents(bag_map: &HashMap<String, Vec<(u64, String)>>, bag_name: &str) -> u64 {
    let (_bag, contents) = bag_map.get_key_value(bag_name).unwrap();
    let sum = contents
        .iter()
        .map(|(contained_bag_count, contained_bag_name)| 1 + contained_bag_count * calc_contents(bag_map, contained_bag_name))
        .sum();
    sum
}