use crate::negation_normal_form;
// use crate::negation_normal_form::get_vars;

pub fn conjunctive_normal_form(formula: &str) -> String {
	let mut f: Vec<char> = negation_normal_form(formula).chars().collect();
    let mut stack: Vec<char> = Vec::new();
	
    for c in &f {
		if *c == '|' {
			if stack.len() > 3 && stack[stack.len() - 2] == '&' {
				let third = f[stack.len() - 1];
				let second = stack[stack.len() - 3];
				let first = stack[stack.len() - 4];
				let new_chars = format!("{}{}|{}{}|&", first, second, first, third); 
				stack.clear();
				stack.extend(new_chars.chars());
			} else {
				stack.push(*c);
			}
		} else {
			stack.push(*c);
		}
	}
	
	f = stack.clone();
	stack.clear();
	let mut vars: Vec<char> = Vec::new();

	for c in &f {
		if *c == '&' || *c == '|' {
			stack.push(*c);
		} else {
			vars.push(*c);
		}
	}
	vars.extend(stack.clone());

	f = vars;
	return f.iter().collect();
}
