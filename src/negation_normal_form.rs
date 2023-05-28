use std::process;

fn clear_formula(f: &mut Vec<char>, formula: &str) {
    for c in formula.chars() {
        if c == '!' {
            if let Some('!') = f.last() {
                f.pop();
            } else {
                f.push(c);
            }
        } else if c == '&' || c == '|' || c == '^' || c == '>' || c == '=' || c.is_ascii_uppercase() {
            f.push(c);
        } else {
            println!("Error: bad RPN syntax");
            process::exit(1);
        }
    }
}

pub fn negation_normal_form(formula: &str) -> String {
	// Cleared formula
    let mut f: Vec<char> = Vec::new();
	// Stack to perform mathematical operations
    let mut stack: Vec<char> = Vec::new();
	// Result string
    let mut res = String::new();

    clear_formula(&mut f, formula);

	for c in f.chars() {
		if c != '!' {
			stack.push(c);
		} else {
			let mut stack2: Vec<char> = Vec::new();
			for &element in stack.iter().rev() {
        		if c == '&' || c == '|' || c == '^' || c == '>' || c == '=' || c.is_ascii_uppercase() {

			}	
		}
	}
	
    return f.into_iter().collect();
}
// res.push_str(&(format!("{}{}&{}!{}!&|", first, second, first, second)).as_str());
