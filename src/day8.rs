use std::{collections::HashSet, str::FromStr, usize};
use lazy_static::lazy_static;
use regex::{Regex};

// compile regex statically so it does not have to be recompiled in loop iterations
lazy_static! {
    static ref re_statement: Regex = Regex::new(r"(\w{3})\s+([+-])(\d+)").unwrap();
}

pub fn day8 () {
    let example_input = std::fs::read_to_string("inputs/d8.example").unwrap();
    let input = std::fs::read_to_string("inputs/d8").unwrap();

    let mut example_vm = Vm::from_script(&example_input);

    for statement in &example_vm.script {
        println!("{:?}", statement);
    }

    println!("Example final acc value: {}", example_vm.run_all())
    // let vm = Vm::from_script(&input);
}

// Let's implement this week's task in a more OO-like fashion instead of the rather functional approach of the last weeks
#[derive(Clone, Debug, Default)]
struct Vm {
    ip_pos: usize,
    previously_executed: HashSet<usize>,
    acc: i64,
    pub script: Vec<Statement>,
    loop_found: bool, // when an endless loop is entered or the end of the script is reached
}

// Missing information in the instructions: What happens when 'jmp' ing outside of the scripts bounds?
impl Vm {
    pub fn from_script(script: &str) -> Self {
        let statements = script
            .lines()
            .map(|line| line.parse::<Statement>().unwrap());

        Self {
            script: statements.collect(),
            ..Default::default()
        }
    }

    // Returns state changes instead of applying them directly to be easily testable
    // returns if a loop was found, the new ip_pos, the new acc value
    fn step(&self) -> (bool, usize, i64) {
        // false if already contained, so negate the value
        let loop_found = !self.previously_executed.insert(self.ip_pos);

        let mut jumped = false;
        let mut new_ip_pos = self.ip_pos;
        let mut new_acc = self.acc;

        let statement = self.script[self.ip_pos];
        match statement.instruction {
            Instruction::Nop => (),
            Instruction::Acc => new_acc += statement.value,
            // instruction pointer is a usize, we have to perform a conversion
            Instruction::Jmp => {
                // if statement.value >= 0 {
                //     self.ip_pos + statement.value
                // } else {
                //     self.ip_pos -= usize::try_from(statement.value.abs())
                // }
                jumped = true;
                new_ip_pos = self.ip_pos.wrapping_add(statement.value as usize);
            },
        }
        if !jumped {
            new_ip_pos += 1;                                  
        }

        println!("{:?}", (loop_found, new_ip_pos, new_acc));

        (loop_found, new_ip_pos, new_acc)
    }

    /// runs the entire script and returns the final acc value
    /// termination happens 
    /// a) at the end of the script
    /// b) when an inifinite loop is entered (a previously visited ip position is visited again) 
    pub fn run_all(&mut self) -> i64 {
        let mut loop_found = false;

        while !loop_found && self.ip_pos < self.script.len() {
            let (l, ip_pos, acc) = self.step();
            loop_found = l;
            self.ip_pos = ip_pos;
            self.acc = acc;
        }
        self.acc
    }
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Nop,
    Acc,
    Jmp,
}

impl FromStr for Instruction {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nop" => Ok(Self::Nop),
            "acc" => Ok(Self::Acc),
            "jmp" => Ok(Self::Jmp),
            _ => Err("Failed to parse instruction")
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Statement {
    instruction: Instruction,
    value: i64,
}

impl FromStr for Statement {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let captures = re_statement.captures(s);
        if let Some(captures) = captures {
            let instruction = captures[1].parse::<Instruction>()?;
            let sign = &captures[2];
            let digit = match captures[3].parse::<u32>() {
                Ok(u32) => u32,
                Err(_) => return Err("Failed to parse integer digit."),
            };

            let value = match sign {
                "+" => i64::from(digit),
                "-" => -1_i64 * i64::from(digit),
                _ => panic!(), // this case is impossible to reach. The other cases are guaranteed to be true based on the regular expression
            };

            Ok( Self { instruction, value})
        } else {
            Err("No match found.")
        }
    }
}

