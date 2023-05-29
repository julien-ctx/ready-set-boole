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

// fn is_operator(c: char) -> bool {
//     if c == '&' || c == '|' || c == '^' || c == '>' || c == '=' || c == '!' {
//         return true;
//     } else {
//         return false;
//     }
// }

pub fn negation_normal_form(formula: &str) -> String {
    let mut f: Vec<char> = Vec::new();
    let mut stack: Vec<char> = Vec::new();

    clear_formula(&mut f, formula);

    while f.iter().collect::<String>().contains("&!") || f.iter().collect::<String>().contains("|!") || f.iter().collect::<String>().contains(">") || f.iter().collect::<String>().contains("^") || f.iter().collect::<String>().contains("="){

        for c in f {
            let len = stack.len();
            if c == '!' {
                if len > 2 && stack[len - 1] < 'A' || stack[len - 1] > 'Z' {
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
            } else if c.is_ascii_uppercase() || c == '&' || c == '|' {
                stack.push(c);
            } else if len >= 2 {
                if c == '>' {
                    stack.insert(len, '|');
                    stack.insert(stack.len() - 2, '!');
                } else if c == '=' {
                    let first_var = stack[len - 2];
                    let second_var = stack[len - 1];
                    let new_chars = format!("{}{}&{}!{}!&|", first_var, second_var, first_var, second_var);
                    stack.pop();
                    stack.pop();
                    stack.extend(new_chars.chars());
                } else if c == '^' {
                    let first_var = stack[len - 2];
                    let second_var = stack[len - 1];
                    let new_chars = format!("{}{}|{}{}&!&", first_var, second_var, first_var, second_var);
                    stack.pop();
                    stack.pop();
                    stack.extend(new_chars.chars()); 
                }
            }
        }
        f = stack.clone();
        stack.clear();
    }
    
    f.iter().collect()
}
