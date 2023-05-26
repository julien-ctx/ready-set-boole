use std::process;

pub fn eval_formula(formula: &str) -> bool {
	let f = formula.replace(" ", "");

	let mut stack: Vec<u8> = Vec::new();

	for c in f.chars() {
		if c.is_numeric() {
			stack.push(c as u8 - 48);
		} else if c == '!' && stack.len() == 1 {
			let index = stack.len() - 1;
			stack[index] = if stack[index] != 0 { 0 } else { 1 };
		} else if stack.len() >= 2 {
			let res;
			match c {
				'&' => res = stack[stack.len() - 2] & stack[stack.len() - 1],
				'|' => res = stack[stack.len() - 2] | stack[stack.len() - 1],
				'^' => res = stack[stack.len() - 2] ^ stack[stack.len() - 1],
				'>' => res = if stack[stack.len() - 2] != 0 && stack[stack.len() - 1] == 0 { 0 } else { 1 },
				'=' => res = (stack[stack.len() - 2] == stack[stack.len() - 1]) as u8,
				_ => {
					println!("Error: forbidden characters in input string");
					process::exit(1);
				}
			}
			stack.pop();
			stack.pop();
			stack.push(res);
		} else {
			println!("Error: bad RPN syntax");
			process::exit(1);	
		}
	}
	return stack[0] != 0;
}
