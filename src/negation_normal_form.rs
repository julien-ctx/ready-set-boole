use std::process;

pub fn negation_normal_form(formula: &str) -> String {
	let f = formula.replace(" ", "");

	let mut stack: Vec<char> = Vec::new();
	let mut res = String::new();

	for c in f.chars() {
		let len = stack.len();

		if c.is_ascii_uppercase()
		// } else {
		// 	println!("Error: bad RPN syntax");
		// 	process::exit(1);
		// }
	}
	return res;
}

// res.push_str(&(format!("{}{}&{}!{}!&|", first, second, first, second)).as_str());
