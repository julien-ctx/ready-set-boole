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
    if c == '&' || c == '|' || c == '^' || c == '>' || c == '=' || c == '!' {
        return true;
    } else {
        return false;
    }
}

fn count_alpha(s: &str) -> u32 {
    let mut count = 0;
    for c in s.chars() {
        if c.is_ascii_uppercase() {
            count += 1;
        }
    }
    count
}

fn create_tokens(f: &[char]) -> Vec<String> {
    let mut stack: Vec<char> = Vec::new();
    let mut tokens: Vec<String> = Vec::new();

    for c in f {
        stack.push(*c);
        if is_operator(*c) {
            tokens.push(stack.iter().collect());
            stack.clear();
        }
    }
    tokens
}

pub fn negation_normal_form(formula: &str) -> String {
    let mut f: Vec<char> = Vec::new();
    let mut stack: Vec<char> = Vec::new();
    let mut tokens: Vec<String> = Vec::new();

    clear_formula(&mut f, formula);

    while f.iter().collect::<String>().contains("&!") || f.iter().collect::<String>().contains("|!") {

        for c in f {
            let len = stack.len();
            if c == '!' {
                if len > 2 && !stack[len - 1].is_ascii_uppercase() {
                    if stack[len - 1] == '&' {
                        stack[len - 1] = '|';
                    } else if stack[len - 1] == '|' {
                        stack[len - 1] = '&';
                    }
                    stack.insert(len - 1, '!');
                    stack.insert(stack.len() - 3, '!');
                } else {
                    stack.push(c);
                }
            } else {
                stack.push(c);
            }
        }
        f = stack.clone();
        stack.clear();
        println!("{:?}", f);
        println!("");
    }
    
    f.iter().collect()
}


// res.push_str(&(format!("{}{}&{}!{}!&|", first, second, first, second)).as_str());
