use std::process;

pub fn eval_formula(formula: &str) -> bool {
	let f = formula.replace(" ", "");

	let mut stack: Vec<u8> = Vec::new();

	for c in f.chars() {
		if c.is_numeric() {
			stack.push(c as u8 - 48);
		} else if stack.len() >= 2 {
			let res;
			match c {
				// '!' => res = stack[stack.len() - 2]!stack[stack.len() - 1],
				'&' => res = stack[stack.len() - 2] & stack[stack.len() - 1],
				'|' => res = stack[stack.len() - 2] | stack[stack.len() - 1],
				'^' => res = stack[stack.len() - 2] ^ stack[stack.len() - 1],
				// '>' => res = stack[stack.len() - 2]>stack[stack.len() - 1],
				// '=' => 
				_ => {
					println!("Error: forbidden characters in input string");
					process::exit(1);
				}
			}
			stack.pop();
			stack.pop();
			stack.push(res);
		}
	}
	return stack[0] != 0;
}
