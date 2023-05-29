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
    let mut f: Vec<char> = Vec::new();
    let mut stack: Vec<char> = Vec::new();

    clear_formula(&mut f, formula);
    
    while f.iter().collect::<String>().contains("&!") || f.iter().collect::<String>().contains("|!") || f.iter().collect::<String>().contains(">") || f.iter().collect::<String>().contains("^") || f.iter().collect::<String>().contains("=") {
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
                    if stack[stack.len() - 3].is_ascii_uppercase() || stack[stack.len() - 3] == '!' {
                        if stack[stack.len() - 3] == '!' {
                            stack.insert(stack.len() - 4, '!')
                        } else {
                            stack.insert(stack.len() - 3, '!')
                        }
                    } else {
                        let mut i = 2;
                        let mut count = 0;
                        while stack.len() - i != 1 {
                            if stack[stack.len() - i].is_ascii_uppercase() {
                                count += 1;
                            }
                            if count == 2 {
                                break;
                            }
                            i += 1;
                        }
                        stack.insert(stack.len() - i, '!');
                    }
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
                    let first_var: Vec<char> = stack.iter().take(len - 1).cloned().collect();
                    let second_var = stack[len - 1]; 
                    let new_chars = format!("{}{}&{}!{}!&|", first_var.iter().collect::<String>(), second_var, first_var.iter().collect::<String>(), second_var);
                    stack.clear();
                    stack.extend(new_chars.chars());
                } else if c == '^' {
                    let first_var: Vec<char> = stack.iter().take(len - 1).cloned().collect();
                    let second_var = stack[len - 1];
                    let new_chars = format!("{}{}|{}!{}!|&", first_var.iter().collect::<String>(), second_var, first_var.iter().collect::<String>(), second_var);
                    stack.clear();
                    stack.extend(new_chars.chars());
                }
            }
        }

        let mut stack_copy: Vec<char> = Vec::new();
        for c in &stack {
            if *c == '!' && stack_copy.len() > 0 {
                if stack_copy[stack_copy.len() - 1] != '!' {
                    stack_copy.push(*c);
                } else {
                    stack_copy.pop();
                }
            } else {
                stack_copy.push(*c);
            }
        }
        f = stack_copy.clone(); 
        
        stack.clear();
    }
    
    f.iter().collect()
}
