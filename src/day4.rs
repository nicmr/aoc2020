use std::{collections::HashMap, fs};

use regex::Regex;

pub fn day4(){
    // let input = fs::read_to_string("inputs/d4.example").unwrap();
    let input = fs::read_to_string("inputs/d4").unwrap();

    let mut passport_strings : Vec<String> = Vec::new();
    // reuse the same string to prevent reallocations
    let mut passport_buffer = String::new();


    let re = Regex::new(r"(\w+):([\w#]+)").unwrap();

    // join lines with contents belonging to a single passport
    // when encountering and empty line, complete parsing of a passport and drop the empty line
    for line in input.lines() {
        if !line.is_empty() {
            // insert a space to prevent concatenation of the last string of a file 
            passport_buffer.push(' ');
            passport_buffer.push_str(line);
        } else {
            // allocate a new string instead of cloning the buffer, cost is the same and buffer might be larger than required
            passport_strings.push(String::from(&passport_buffer)); 
            passport_buffer.clear();
        }
    }
    // make sure a passport in the end without a trailing empty line is not dropped
    if !passport_buffer.is_empty() {
        passport_strings.push(passport_buffer);
    }

    

    let required_keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    // construct hashmap for each passport with all values
    let passport_maps : Vec<HashMap<String,String>> = passport_strings
        .iter()
        .map(|s| {
            let mut map: HashMap<String, String> = HashMap::new();
            s
                .split(' ')
                .for_each(|kv| {
                    println!("trying to use regex on str:'{}'", kv);
                    if let Some(captures) = re.captures(kv){
                        map.insert(captures[1].to_owned(), captures[2].to_owned());
                    }
                    // contents that do not match the regex will be dropped                    
                });
            map
        }).collect();

    // count the maps which contain the required values

    let correct_passports = passport_maps
        .iter()
        .filter(|hm| {
            let mut missing_field = false;
            for k in required_keys.iter() {
                if !hm.contains_key(*k) {
                    println!("missing key '{}'", *k);
                    println!("hashmap contents:{:?}", hm);
                    missing_field = true;
                    break;
                }
            }
            missing_field
            
            // all(|k| hm.contains_key(*k))
        })
        .count();
    
    println!("Total passport count: {}", passport_maps.len());
    println!("Correct passport count: {}", correct_passports);

    // for passport_str in passport_strings {
    //     println!("{}", passport_str);
    //     println!("end");
    // }    
}
