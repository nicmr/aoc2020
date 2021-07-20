use std::{fs, str, u64};
use regex::Regex;

#[derive(Clone, Debug)]
struct PasswordRule {
    pub rule_letter: char,
    pub rule_min: u64,
    pub rule_max: u64,
    pub password: String,
}

impl PasswordRule {
    /// Attempts to parse passworld rules from the specified string. Panics on failure.
    fn try_parse_str(input: &str ) -> Option<Self> {
        let re = Regex::new(r"(\d+)-(\d+)\s([a-z]):\s([a-z]+)").unwrap();
        // only consider first match:
        match re.captures(input) {
            Some(cap) => {
                Some(Self {
                    rule_min: cap[1].parse().unwrap(),
                    rule_max: cap[2].parse().unwrap(),
                    rule_letter: cap[3].chars().next().unwrap(),
                    password: cap[4].to_owned(),
                })
            },
            None => {
                println!("Failed to parse {}", input);
                None
            },
        }
    }

    fn validate_task_1(&self) -> bool {
        let occurences = self.password.chars().filter(|c| c == &self.rule_letter).count() as u64;
        occurences >= self.rule_min && occurences <= self.rule_max 
    }

    fn validate_task_2(&self) -> bool {
        let chars : Vec<char> = self.password.chars().collect();
        (chars[self.rule_min as usize -1] == self.rule_letter) ^ (chars[self.rule_max as usize -1] == self.rule_letter)
    }
}

pub fn day2(){
    let input = fs::read_to_string("inputs/d2").unwrap();
    let values : Vec<PasswordRule> =
        input
            .lines()
            .filter_map(|s| PasswordRule::try_parse_str(s))
            .collect();

    let solution1  =
        values.clone()
            .into_iter()
            .filter(|rule| rule.validate_task_1());
            // do not collect to prevent allocation


    println!("Count of rule-abiding passwords for task1: {}", solution1.count());

    let solution2 =
        values.into_iter()
            .filter(|rule| rule.validate_task_2());
            // do not collect to prevent allocation
 
    println!("Count of rule-abiding passwords for task2: {}", solution2.count());
}



// I'd been looking for a reason to try out the parser combinator library nom for a while. 
// In the end, I found it rather unergonomic and verbose compared to Haskell's parsec (and its derivatives).
// So I just used regex to get on with the next challenges.

// use nom::{IResult, bytes::complete::{tag, take_while}, character::{complete::{anychar}}, combinator::map_res};

// fn from_limit(input: &str) -> Result<u64, ParseIntError> {
//     u64::from_str_radix(input, 10)
// }

// fn parse_limit(input: &str) -> IResult<&str, u64> {
//     map_res(
//         take_while( char::is_ascii_digit),
//         from_limit
//     )(input)

//     // let (input, digits) = take_while(char::is_ascii_digit)(input);
//     // let u = from_limit(digits);
//     // Ok((input, ))
// }

// fn parse_password(input: &str) -> IResult<&str, String> {
//     let (input, alphabetic) = take_while(char::is_alphabetic)(input)?;
//     Ok((input, alphabetic.to_string()))
// }

// fn parse_password_rule(input: &str) -> IResult<&str, PasswordRule> {
//     let (input, rule_min) = parse_limit(input)?;
//     let (input, _) = tag("-")(input)?;
//     let (input, rule_max) = parse_limit(input)?;
//     let (input, _) = tag(" ")(input)?;
//     let (input, rule_letter) = anychar(input)?;
//     let (input, _) = tag(": ")(input)?;
//     let (input, password) = parse_password(input)?;

//     Ok((input,
//         PasswordRule {
//             rule_letter,
//             rule_min,
//             rule_max,
//             password,
//         }
//     ))
// }