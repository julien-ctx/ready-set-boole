mod adder;
mod multiplier;
mod gray_code;
mod eval_formula;
mod print_truth_table;
mod negation_normal_form;
mod conjunctive_normal_form;
mod sat;
mod powerset;
mod eval_set;
mod map;
mod reverse_map;

use adder::adder;
use multiplier::multiplier;
use gray_code::gray_code;
use eval_formula::eval_formula;
use print_truth_table::print_truth_table;
use negation_normal_form::negation_normal_form;
use conjunctive_normal_form::conjunctive_normal_form;
use sat::sat;
use powerset::powerset;
use eval_set::eval_set;
use map::map;
use reverse_map::reverse_map;

use rand::Rng;

fn main() {
    let bold = "\x1b[37m";
    let reset = "\x1b[0m";
    // Add tool to generate random numbers
    let mut rng = rand::thread_rng();

	println!("\n{}--------------------EXERCISE 00--------------------{}", bold, reset);
    for _ in 0..15 {
        let rand1: u32 = rng.gen_range(0..=1000);
        let rand2: u32 = rng.gen_range(0..=1000);
        let my_result = adder(rand1, rand2);
        let real_result = rand1 + rand2;
        let diff = if my_result == real_result { "\x1b[32mOK\x1b[0m" } else { "\x1b[31mKO\x1b[0m" };
        println!("My result: {}{}{} | Real result: {}{}{} | DIFF {}", bold, my_result, reset, bold, real_result, reset, diff);
    }
    // Tricky cases:
    let my_result = adder(42, 42);
    let real_result = 42 + 42;
    let diff = if my_result == real_result { "\x1b[32mOK\x1b[0m" } else { "\x1b[31mKO\x1b[0m" };
    println!("My result: {}{}{} | Real result: {}{}{} | DIFF {}", bold, my_result, reset, bold, real_result, reset, diff); 

    let my_result = adder(0, 42);
    let real_result = 0 + 42;
    let diff = if my_result == real_result { "\x1b[32mOK\x1b[0m" } else { "\x1b[31mKO\x1b[0m" };
    println!("My result: {}{}{} | Real result: {}{}{} | DIFF {}", bold, my_result, reset, bold, real_result, reset, diff); 

	println!("\n{}--------------------EXERCISE 01--------------------{}", bold, reset);
	for _ in 0..15 {
        let rand1: u32 = rng.gen_range(0..=500);
        let rand2: u32 = rng.gen_range(0..=500);
        let my_result = multiplier(rand1, rand2);
        let real_result = rand1 * rand2;
        let diff = if my_result == real_result { "\x1b[32mOK\x1b[0m" } else { "\x1b[31mKO\x1b[0m" };
        println!("My result: {}{}{} | Real result: {}{}{} | DIFF {}", bold, my_result, reset, bold, real_result, reset, diff);
    }

	 // Tricky cases:
	 let my_result = multiplier(42, 42);
	 let real_result = 42 * 42;
	 let diff = if my_result == real_result { "\x1b[32mOK\x1b[0m" } else { "\x1b[31mKO\x1b[0m" };
	 println!("My result: {}{}{} | Real result: {}{}{} | DIFF {}", bold, my_result, reset, bold, real_result, reset, diff); 
 
	 let my_result = multiplier(0, 42);
	 let real_result = 0 * 42;
	 let diff = if my_result == real_result { "\x1b[32mOK\x1b[0m" } else { "\x1b[31mKO\x1b[0m" };
	 println!("My result: {}{}{} | Real result: {}{}{} | DIFF {}", bold, my_result, reset, bold, real_result, reset, diff); 

    0;
	
	println!("\n{}--------------------EXERCISE 02--------------------{}", bold, reset);
	let numbers: [u32; 8] = [0, 854, 1, 54, 42, 99, 100, 5];
	let results: [u32; 8] = [0, 765, 1, 45, 63, 82, 86, 7];
	for (number, result) in numbers.iter().zip(results.iter()) {
		let my_result = gray_code(*number);
		let diff = if my_result == *result { "\x1b[32mOK\x1b[0m" } else { "\x1b[31mKO\x1b[0m" };	
	 	println!("My result: {}{}{} | Real result: {}{}{} | DIFF {}", bold, my_result, reset, bold, *result, reset, diff); 
	}

    println!("\n{}--------------------EXERCISE 03--------------------{}", bold, reset);
    let tests: [&str; 9] = ["10&", "10|", "11>", "10=", "1011||=",    "101110^=>|&!", "1!!!", "1100|^&", "00101&>^|"];
    let results: [bool; 9] = [false, true, true, false, true,    false, false, true, false];
    for (test, result) in tests.iter().zip(results.iter()) {
        let my_result = eval_formula(*test);
		let diff = if my_result == *result { "\x1b[32mOK\x1b[0m" } else { "\x1b[31mKO\x1b[0m" };	
	 	println!("My result: {}{}{} | Real result: {}{}{} | DIFF {}", bold, my_result, reset, bold, *result, reset, diff); 
    }

    println!("\n{}--------------------EXERCISE 04--------------------{}", bold, reset);
    print_truth_table("AB&C|");
    println!("");
    print_truth_table("AA&A=");
    println!("");
    print_truth_table("AB^!");

    println!("\n{}--------------------EXERCISE 05--------------------{}", bold, reset);
    println!("{}\n", negation_normal_form("AB|!")); // A!B!&
    println!("{}\n", negation_normal_form("AB>")); // A!B|
    println!("{}\n", negation_normal_form("AB=")); // AB&A!B!&|
    println!("{}\n", negation_normal_form("AB|C&!")); // A!B!&C!|
    println!("{}\n", negation_normal_form("AB^")); // AB|A!B!|&
    println!("{}\n", negation_normal_form("AB=C=")); // AB&A!B!&|C&A!B!|AB|&C!&|
    println!("{}\n", negation_normal_form("AB^C^")); // AB|A!B!|&C|A!B!&AB&|C!|&

    println!("\n{}--------------------EXERCISE 06--------------------{}", bold, reset);
    println!("{}\n", conjunctive_normal_form("AB&!")); // A!B!|
    println!("{}\n", conjunctive_normal_form("AB|!")); // A!B!&
    println!("{}\n", conjunctive_normal_form("AB|C&")); // AB|C&
    println!("{}\n", conjunctive_normal_form("AB|C|D|")); // ABCD|||
    println!("{}\n", conjunctive_normal_form("AB&C&D&")); // ABCD&&&
    println!("{}\n", conjunctive_normal_form("AB&!C!|")); // A!B!C!||
    println!("{}\n", conjunctive_normal_form("AB|!C!&")); // A!B!C!&&
    println!("{}\n", conjunctive_normal_form("ABCD&|&")); // ABC|BD|&&

    println!("\n{}--------------------EXERCISE 07--------------------{}", bold, reset);
    println!("{}", sat("AB|")); // true
    println!("{}", sat("AB&")); // true
    println!("{}", sat("AA!&")); // false
    println!("{}", sat("AA^")); // false
    println!("{}", sat("A!A^")); // false
    println!("{}", sat("A!A^!")); // false
    
    println!("\n{}--------------------EXERCISE 08--------------------{}", bold, reset);
    println!("{:?}", powerset(&[1]));
    println!("{:?}", powerset(&[1, 2]));
    println!("{:?}", powerset(&[1, 2, 3]));

    println!("\n{}--------------------EXERCISE 09--------------------{}", bold, reset);
    let sets: [&[i32]; 2] = [
        &[0, 1, 2],
        &[0, 3, 4],
    ];
    let result = eval_set("AB&", &sets);
    println!("{:?}", result);
    // [0]

    let sets: [&[i32]; 2] = [
        &[0, 1, 2],
        &[3, 4, 5],
    ];
    let result = eval_set("AB|", &sets);
    println!("{:?}", result);
    // [0, 1, 2, 3, 4, 5]
    
    let sets: [&[i32]; 2] = [
        &[0, 1, 2],
        &[1, 2, 3],
    ];
    let result = eval_set("AB^", &sets);
    println!("{:?}", result);
    // [0, 3]

    let sets: [&[i32]; 2] = [
        &[0, 1, 2],
        &[0, 1, 2],
    ];
    let result = eval_set("AB=", &sets);
    println!("{:?}", result);
    // [0, 1, 2]

    let sets: [&[i32]; 2] = [
        &[0, 1, 6],
        &[0, 1, 2, 3, 5],
    ];
    let result = eval_set("AB>", &sets);
    println!("{:?}", result);
    // []

    let sets: [&[i32]; 1] = [
        &[1, 2, 3],
    ];
    let result = eval_set("A!", &sets);
    println!("{:?}", result);
    // []

    let sets: [&[i32]; 2] = [
        &[1, 2, 3],
        &[1, 2, 3],
    ];
    let result = eval_set("AB=A=", &sets);
    println!("{:?}", result);
    // [1, 2, 3]

    println!("\n{}--------------------EXERCISE 10--------------------{}", bold, reset);
    println!("{}", map(5, 6)); // 57
    println!("{}", map(0, 3)); // 10
    println!("{}", map(21, 42)); // 2457
    
    println!("{}", map(65535, 65535));
    println!("{}", map(65535, 32767));
    
    println!("\n{}--------------------EXERCISE 11--------------------{}", bold, reset);
    println!("{:?}", reverse_map(0.000000013271346691360545));
    println!("{:?}", reverse_map(0.0000000023283064370807974));
    println!("{:?}", reverse_map(0.0000005720648915907519));

    println!("{:?}", reverse_map(1.0));
    println!("{:?}", reverse_map(0.5));
}
