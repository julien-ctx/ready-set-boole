use crate::eval_formula;

use std::process;

fn calculate_combination(f: &mut String, vars: Vec<char>, permutation: &mut Vec<char>, permutation_len: u32) {
	let mut formula = f.clone();
	if permutation_len == vars.len() as u32 {
		for (var, perm) in vars.iter().zip(permutation.iter()) {
			formula = formula.replace(&var.to_string(), &perm.to_string());
		}
		let equivalence = eval_formula(formula.as_str());
		permutation.push(if equivalence { '1' } else { '0' });
		print_row(permutation, false);
		permutation.pop();
	} else {
		for i in 0..2 {
			permutation[permutation_len as usize] = char::from_u32(i + 48).unwrap();
			calculate_combination(f, vars.clone(), permutation, (permutation_len + 1) as u32);
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
	let mut f = String::from(formula.replace(" ", ""));
	
	let mut vars: Vec<char> = Vec::new();
	for c in formula.chars() {
		if c.is_ascii_uppercase() && c >= 'A' && c <= 'Z' {
			if !vars.contains(&c) {
				vars.push(c);
			}
		}
	}
	// Vars are all the variables in formula.
	if vars.len() == 0 {
		println!("Error: no variables in input string");
		process::exit(1);
	}
	
	// Permutation is a row in the table.
	let mut permutation: Vec<char> = Vec::new();
	for _ in &vars {
		permutation.push('0');
	}
	// Print header
	print_row(&vars, true);
	calculate_combination(&mut f, vars.clone(), &mut permutation, 0);
}
