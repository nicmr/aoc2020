#![feature(fn_traits)]

use std::{collections::HashMap, env};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

fn main() {
    let mut daymap: HashMap<String, Box<dyn Fn()>> = HashMap::new();

    // boilerplate so each days solution can be called using a command line option
    // A macro to handle this would be really great
    daymap.insert("1".to_string(), Box::new(day1::day1));
    daymap.insert("2".to_string(), Box::new(day2::day2));
    daymap.insert("3".to_string(), Box::new(day3::day3));
    daymap.insert("4".to_string(), Box::new(day4::day4));
    daymap.insert("5".to_string(), Box::new(day5::day5));
    daymap.insert("6".to_string(), Box::new(day6::day6));
    daymap.insert("7".to_string(), Box::new(day7::day7));
    daymap.insert("8".to_string(), Box::new(day8::day8));

    let args : Vec<String> = env::args().collect();
    if args.len() >= 2 {
        match daymap.get(&args[1]) {
            Some(day_fn) => day_fn.call(()),
            None => println!("fn not found"),
        }
    } else {
        println!("Please pass the day you would like to execute the solution of as an argument");
    }
}