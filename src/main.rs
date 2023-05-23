mod adder;

use adder::adder;
use rand::Rng;

fn main() {
    let bold = "\x1b[37m";
    let reset = "\x1b[0m";
    // Add tool to generate random numbers
    // let mut rng = rand::thread_rng();

	// println!("{}--------------------EXERCISE 00--------------------{}", bold, reset);
    // for _ in 0..15 {
    //     let rand1: u32 = rng.gen_range(0..=1000);
    //     let rand2: u32 = rng.gen_range(0..=1000);
    //     let my_result = adder(rand1, rand2);
    //     let real_result = rand1 + rand2;
    //     let diff = if my_result == real_result { "\x1b[32mOK\x1b[0m" } else { "\x1b[31mKO\x1b[0m" };
    //     println!("My result: {}{}{} | Real result: {}{}{} | DIFF {}", bold, my_result, reset, bold, real_result, reset, diff);
    // }
    // // Tricky cases:
    // let my_result = adder(42, 42);
    // let real_result = 42 + 42;
    // let diff = if my_result == real_result { "\x1b[32mOK\x1b[0m" } else { "\x1b[31mKO\x1b[0m" };
    // println!("My result: {}{}{} | Real result: {}{}{} | DIFF {}", bold, my_result, reset, bold, real_result, reset, diff); 

    // let my_result = adder(0, 42);
    // let real_result = 0 + 42;
    // let diff = if my_result == real_result { "\x1b[32mOK\x1b[0m" } else { "\x1b[31mKO\x1b[0m" };
    // println!("My result: {}{}{} | Real result: {}{}{} | DIFF {}", bold, my_result, reset, bold, real_result, reset, diff); 

	println!("{}--------------------EXERCISE 01--------------------{}", bold, reset);
	
    0;
}
