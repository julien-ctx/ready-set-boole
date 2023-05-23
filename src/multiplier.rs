mod adder;

use adder::adder;

fn generate_nb(value: u32) -> u32 {
    return if value == 1 {4294967295} else {0};
}

fn multiplier(a: u32, b: u32) -> u32 {
    let mut numbers: Vec<u32> = Vec::new();
    for i in 0..32 {
        let mask: u32 = generate_nb(b >> i & 1);
        numbers.push((a & mask) << i);
    }
    for num in &numbers {
        println!("{:32b} {}", num, num);
    }
    a
}

fn main() {
	// println!("{}", multiplier(29, 9));
    multiplier(29, 9);
    0;
}
