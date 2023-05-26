use crate::eval_formula;

fn get_permutations(vars: Vec<char>, permutation: &mut Vec<u32>, permutation_len: u32) {
	if permutation_len == vars.len() as u32 {
		// println!("{:?}", permutation);
		return;
	} else {
		for i in 0..2 {
			permutation[permutation_len as usize] = i as u32;
			get_permutations(vars.clone(), permutation, (permutation_len + 1) as u32);
		}
	}
}

fn get_truth_table(formula: &str) -> Vec<char> {
	let vars: Vec<char> = formula
	.chars()
	.filter(|&c| (c.is_ascii() && (c as u8) >= 65 && (c as u8) <= 90))
	.collect();
	let mut permutation: Vec<u32> = Vec::new();
	for _ in &vars {
		permutation.push(0);
	}
	get_permutations(vars.clone(), &mut permutation, 0);
	println!("{:?}", permutation);
	vars
}

pub fn print_truth_table(formula: &str) {
	let f = formula.replace(" ", "");
	let truth_table = get_truth_table(formula);
	// for row in &truth_table {
	// 	println!("{}", *row);
	// }
}
