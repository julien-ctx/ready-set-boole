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

fn is_operator(c: char) -> bool {
    if c == '&' || c == '|' || c == '^' || c == '>' || c == '=' {
        return true;
    } else {
        return false;
    }
}

pub fn negation_normal_form(formula: &str) -> String {
    let mut f: Vec<char> = Vec::new();
    let mut stack: Vec<char> = Vec::new();
    let mut tokens: Vec<String> = Vec::new();

    clear_formula(&mut f, formula);

    for c in f {
        stack.push(c);
        if is_operator(c) {
            tokens.push(stack.iter().collect());
            stack.clear();
        }
    }   

    for token in &tokens {
        println!("{}", token);
    }

    stack.iter().collect()
}


// res.push_str(&(format!("{}{}&{}!{}!&|", first, second, first, second)).as_str());
