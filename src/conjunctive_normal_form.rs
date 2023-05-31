use crate::negation_normal_form;

fn get_vars(stack: &mut Vec<char>) -> (Vec<char>, Vec<char>, Vec<char>, Vec<char>) {
    let mut copy = stack.clone();
    let mut first = Vec::new();
    let mut second = Vec::new();
    let mut third = Vec::new();
	let mut rest = Vec::new();

    copy.pop();
	let mut i = 3;
    for c in copy.iter().rev() {
		if i == 3 {
			third.insert(0, *c);
		} else if i == 2 {
			second.insert(0, *c);
		} else if i == 1 {
			first.insert(0, *c);
		} else {
			rest.insert(0, *c);
		}
		if (*c).is_ascii_uppercase() {
			i -= 1;
		}
	}
	return (rest, first, second, third);
}

pub fn conjunctive_normal_form(formula: &str) -> String {
	let mut f: Vec<char> = negation_normal_form(formula).chars().collect();
    let mut stack: Vec<char> = Vec::new();
    let mut vars: Vec<char> = Vec::new();
	
	for c in &f {
		if stack.len() > 3 && stack[stack.len() - 1] == '&' && *c == '|' {
			let (rest, first, second, third) = get_vars(&mut stack);
			let new_chars = format!("{}{}{}|{}{}|&", rest.iter().collect::<String>(), first.iter().collect::<String>(), second.iter().collect::<String>(), first.iter().collect::<String>(), third.iter().collect::<String>());
			stack.clear();
			stack.extend(new_chars.chars());
		} else {
			stack.push(*c);
		}
	}
	f = stack.clone();
	stack.clear();

	let mut operator: char = '|';
	if f.iter().collect::<String>().contains("&") {
		operator = '&';
	}
	for c in &f {
		if *c == operator {
			stack.push(*c);
		} else {
			vars.push(*c);
		}
	}
	vars.extend(stack.clone());

	f = vars;
	return f.iter().collect();
}
