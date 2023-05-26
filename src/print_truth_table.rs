use crate::eval_formula;
use crate::adder;

use std::process;
use std::collections::HashSet;

fn calculate_combination(f: &str, vars: Vec<char>, permutation: &mut Vec<u32>, permutation_len: u32) {
	// If permutation has been fully filled.
	if permutation_len == vars.len() as u32 {
		let mut i: u32 = 0;
		let mut formula: Vec<char> = f.chars().collect();
		for c in &mut formula {
			if *c >= 'A' && *c <= 'Z' {
				*c = char::from_u32(permutation[i as usize] as u32 + 48).unwrap();
				i = adder(i, 1);
			}
		}
		let equivalence = eval_formula(formula.iter().collect::<String>().as_str());
		if equivalence {
			permutation.push(1);
		} else {
			permutation.push(0);
		}
		print_row(permutation, false);
		permutation.pop();
	} else {
		for i in 0..2 {
			permutation[permutation_len as usize] = i as u32;
			calculate_combination(&f, vars.clone(), permutation, (permutation_len + 1) as u32);
		}
	}
}

fn print_row<T>(vars: &Vec<T>, header: bool)
where
    T: std::fmt::Display,
{
    print!("|");
    for item in vars {
        print!(" {} |", item);
    }
    if header {
        print!(" = |\n|");
		for _ in 0..=vars.len() {
			print!("---|");
		}
    }
	println!("");
}

pub fn print_truth_table(formula: &str) {
	let f = formula.replace(" ", "");

	let all_vars: Vec<char> = formula
	.chars()
	.filter(|&c| (c.is_ascii() && (c as u8) >= 65 && (c as u8) <= 90))
	.collect();
	// Remove duplicates
	let vars: Vec<char> = all_vars.into_iter().collect::<HashSet<_>>().into_iter().collect();
	if vars.len() == 0 {
		println!("Error: no variables in input string");
		process::exit(1);
	}
	
	let mut permutation: Vec<u32> = Vec::new();
	for _ in &vars {
		permutation.push(0);
	}

	print_row(&vars, true);
	calculate_combination(&f, vars.clone(), &mut permutation, 0);
}
