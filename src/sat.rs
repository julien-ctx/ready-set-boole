use crate::eval_formula;
use crate::adder;

use std::process;

fn calculate_combination(f: &mut String, vars: Vec<char>, permutation: &mut Vec<char>, permutation_len: u32) -> bool{
	let mut formula = f.clone();
	if permutation_len == vars.len() as u32 {
		for (var, perm) in vars.iter().zip(permutation.iter()) {
			formula = formula.replace(&var.to_string(), &perm.to_string());
		}
		let equivalence = eval_formula(formula.as_str());
		return equivalence;
	} else {
		for i in 0..2 {
			permutation[permutation_len as usize] = char::from_u32(adder(i, 48)).unwrap();
			let res = calculate_combination(f, vars.clone(), permutation, adder(permutation_len, 1));
			if res {
				return true;
			}
		}
	}
	return false;
}

pub fn sat(formula: &str) -> bool {
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

	let mut permutation: Vec<char> = Vec::new();
	for _ in &vars {
		permutation.push('0');
	}
	return calculate_combination(&mut f, vars.clone(), &mut permutation, 0);
}
