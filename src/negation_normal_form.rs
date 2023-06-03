use crate::adder;
use std::process;

fn is_binary_operator(c: char) -> bool {
    c == '&' || c == '|' || c == '^' || c == '>' || c == '='
}

fn remove_double_negation(vec: &mut Vec<char>) {
    let copy = vec.clone();
    vec.clear();
    for c in copy {
        if c == '!' {
            if vec.len() > 0 && vec[vec.len() - 1] == '!' {
                vec.pop();
            } else if vec.len() > 0 {
                vec.push(c);
            } else {
                println!("Error: bad input format");
                process::exit(1); 
            }
        } else if c.is_ascii_uppercase() || is_binary_operator(c) {
            vec.push(c);
        } else {
            println!("Error: bad input format");
            process::exit(1);
        }
    }
}

fn contains_forbidden_chars(f: &mut Vec<char>) -> bool {
    f.iter().collect::<String>().contains("&!") || f.iter().collect::<String>().contains("|!") || f.iter().collect::<String>().contains("=") || f.iter().collect::<String>().contains(">") || f.iter().collect::<String>().contains("^")
}

fn get_vars(stack: &mut Vec<char>) -> (Vec<char>, Vec<char>) {
    let mut copy = stack.clone();
    copy.pop();

    let mut i = 0;
    if is_binary_operator(copy[copy.len() - 1]) {
        let mut alpha_count = 0;
        for (index, c) in copy.iter().enumerate().rev() {
            i = index;
            if c.is_ascii_uppercase() {
                alpha_count = adder(alpha_count, 1);
            }
            if alpha_count == 2 {
                break;
            }
        }
        return ((&stack[0..i]).to_vec(), (&stack[i..stack.len() - 1]).to_vec());
    } else if copy[copy.len() - 1] == '!' {
        return ((&stack[0..stack.len() - 3]).to_vec(), (&stack[stack.len() - 3..stack.len() - 1]).to_vec())
    } else {
        return ((&stack[0..stack.len() - 2]).to_vec(), (&stack[stack.len() - 2..stack.len() - 1]).to_vec());
    }
}

pub fn negation_normal_form(formula: &str) -> String {
    let mut f: Vec<char> = formula.chars().collect();
    let mut stack: Vec<char> = Vec::new();
    
    remove_double_negation(&mut f);
    while contains_forbidden_chars(&mut f) {
        for c in &f {
            let len = stack.len();
            if *c == '!' {
                if len > 0 && stack[len - 1] == '&' || stack[len - 1] == '|' {
                    if stack[len - 1] == '&' {
                        stack[len - 1] = '|';
                    } else {
                        stack[len - 1] = '&';
                    }
                    let (first, second) = get_vars(&mut stack);
                    let new_chars = format!("{}!{}!{}", first.iter().collect::<String>(), second.iter().collect::<String>(), stack[len - 1]);
                    stack.clear();
                    stack.extend(new_chars.chars());
                } else {
                    stack.push(*c); 
                }
            } else if *c == '=' || *c == '>' || *c == '^' {
                stack.push(*c); 
                let (first, second) = get_vars(&mut stack);
                let new_chars;
                if *c == '=' {
                    new_chars = format!("{}{}&{}!{}!&|", first.iter().collect::<String>(), second.iter().collect::<String>(), first.iter().collect::<String>(), second.iter().collect::<String>());
                } else if *c == '>' {
                    new_chars = format!("{}!{}|", first.iter().collect::<String>(), second.iter().collect::<String>());
                } else {
                    new_chars = format!("{}{}|{}!{}!|&", first.iter().collect::<String>(), second.iter().collect::<String>(), first.iter().collect::<String>(), second.iter().collect::<String>());
                }
                stack.clear();
                stack.extend(new_chars.chars());
            } else {
                stack.push(*c);
            }
        } 
        f = stack.clone();
        stack.clear();
        remove_double_negation(&mut f);
    }
    f.iter().collect()
}
