use rand::Rng;

fn adder(a: u32, b: u32) -> u32 {
    let mut result = 0;
    let mut carry_over = 0;

    for i in 0..32 {
        let a_bit = a >> i & 1;
        let b_bit = b >> i & 1;

		// Check if only one value is 1, or all, which is necessary to set the result binary to 1.
        let binary = a_bit ^ b_bit ^ carry_over;
		// Check if 2 or 3 values are 1, which is the condition to set carry_over to 1.
        carry_over = (a_bit & b_bit) | (a_bit & carry_over) | (b_bit & carry_over);
		// If binary_pos is 0, it doesn't do anything. Otherwise, it set the correct bit to 1.
        result |= binary << i;
    }
    result
}

fn main() {
    let bold = "\x1b[37m";
    let reset = "\x1b[0m";
    // Add tool to generate random numbers
    let mut rng = rand::thread_rng();

    for _ in 0..15 {
        let rand1: u32 = rng.gen_range(0..=1000);
        let rand2: u32 = rng.gen_range(0..=1000);
        let my_result = adder(rand1, rand2);
        let real_result = rand1 + rand2;
        let diff = if my_result == real_result { "\x1b[32mOK\x1b[0m" } else { "\x1b[31mKO\x1b[0m" };
        println!("My result: {}{}{} | Real result: {}{}{} | DIFF {}", bold, my_result, reset, bold, real_result, reset, diff);
    }
    // Tricky cases:
    let my_result = adder(42, 42);
    let real_result = 42 + 42;
    let diff = if my_result == real_result { "\x1b[32mOK\x1b[0m" } else { "\x1b[31mKO\x1b[0m" };
    println!("My result: {}{}{} | Real result: {}{}{} | DIFF {}", bold, my_result, reset, bold, real_result, reset, diff); 

    let my_result = adder(0, 42);
    let real_result = 0 + 42;
    let diff = if my_result == real_result { "\x1b[32mOK\x1b[0m" } else { "\x1b[31mKO\x1b[0m" };
    println!("My result: {}{}{} | Real result: {}{}{} | DIFF {}", bold, my_result, reset, bold, real_result, reset, diff); 

    0;
}
