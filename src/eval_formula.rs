use std::process;

fn logical_negation(nb: u8) -> u8 {
	1 - nb
}

pub fn eval_formula(formula: &str) -> bool {
	let f = formula.replace(" ", "");

	let mut stack: Vec<u8> = Vec::new();

	for c in f.chars() {
		if c == '1' || c == '0' {
			stack.push(c as u8 - 48);
		} else if c == '!' {
			let index = stack.len() - 1;
			stack[index] = logical_negation(stack[index]);
		} else if stack.len() >= 2 {
			let res;
			match c {
				'&' => res = stack[stack.len() - 2] & stack[stack.len() - 1],
				'|' => res = stack[stack.len() - 2] | stack[stack.len() - 1],
				'^' => res = stack[stack.len() - 2] ^ stack[stack.len() - 1],
				'>' => res = logical_negation(stack[stack.len() - 2]) | stack[stack.len() - 1],
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
	if stack.len() != 1 {
		println!("Error: bad RPN syntax");
		process::exit(1);	
	}
	return stack[0] != 0;
}
