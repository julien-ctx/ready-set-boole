use std::process;

fn logical_negation(set: &[i32]) -> Vec<i32> {
	if set.to_vec().len() != 0 {
		return Vec::new();
	} else {
		return vec![1];
	}
}

fn and(first: &[i32], second: &[i32]) -> Vec<i32> {
	let mut res: Vec<i32> = Vec::new();
	for (num1, num2) in first.iter().zip(second.iter()) {
		if *num1 == *num2 {
			res.push(*num1);
		}
	}
	res
}

fn or(first: &[i32], second: &[i32]) -> Vec<i32> {
	let mut res: Vec<i32> = Vec::new();
	for (num1, num2) in first.iter().zip(second.iter()) {
		if !res.contains(num1) {
			res.push(*num1);
		}
		if !res.contains(num2) {
			res.push(*num2);
		}
	}
	res
}

fn xor(first: &[i32], second: &[i32]) -> Vec<i32> {
	let mut res: Vec<i32> = Vec::new();
	for (num1, num2) in first.iter().zip(second.iter()) {
		if !second.contains(num1) {
			res.push(*num1);
		}
		if !first.contains(num2) {
			res.push(*num2);
		}
	}
	res
}

fn equivalence(first: &[i32], second: &[i32]) -> Vec<i32> {
	if first.len() != second.len() {
		return Vec::new();
	}
	for (num1, num2) in first.iter().zip(second.iter()) {
		if *num1 != *num2 {
			return Vec::new();
		}
	}
	first.to_vec()
}

fn implication(first: &[i32], second: &[i32]) -> Vec<i32> {
	for &num in first {
		if !second.contains(&num) {
			return Vec::new();
		}
	}
	second.to_vec()
}

pub fn eval_set(formula: &str, sets: &[&[i32]]) -> Vec<i32> {
	let f = formula.replace(" ", "");
	let mut stack: Vec<Vec<i32>> = Vec::new();
	let mut tmp: Vec<i32> = Vec::new();
	let mut i: u32 = 0;
	let mut vars: Vec<char> = Vec::new();
	
	for c in f.chars() {
		if c.is_ascii_uppercase() {
			if let Some(index) = vars.iter().position(|&ch| ch == c) {
				stack.push(sets[index as usize].to_vec());
			} else {
				stack.push(sets[i as usize].to_vec());
				vars.push(c);
				i += 1;
			}
		} else if c == '!' {
			tmp.extend(logical_negation(&stack[stack.len() - 1]));
			stack.pop();
			stack.push(tmp.clone());
			tmp.clear();
		} else if stack.len() >= 2 {
			match c {
				'&' => tmp.extend(and(&stack[stack.len() - 2], &stack[stack.len() - 1])),
				'|' => tmp.extend(or(&stack[stack.len() - 2], &stack[stack.len() - 1])),
				'^' => tmp.extend(xor(&stack[stack.len() - 2], &stack[stack.len() - 1])),
				'=' => tmp.extend(equivalence(&stack[stack.len() - 2], &stack[stack.len() - 1])),
				'>' => tmp.extend(implication(&stack[stack.len() - 2], &stack[stack.len() - 1])),
				_ => {
					println!("Error: forbidden characters in input string");
					process::exit(1);
				}
			}
			stack.pop();
			stack.pop();
			stack.push(tmp.clone());
			tmp.clear();
		} else {
			println!("Error: bad RPN syntax");
			process::exit(1);	
		}
	}
	if stack.len() == 0 {
		println!("Error: bad RPN syntax");
		process::exit(1);
	}
	stack[0].clone()
}
