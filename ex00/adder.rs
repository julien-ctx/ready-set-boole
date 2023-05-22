fn set_bit(nb: u32, bit_nb: u32) -> u32 {
	return (1 << bit_nb) | nb;
}

fn adder(a: u32, b: u32) -> u32 {
	let mut result: u32 = 0;
	let mut carry_over: u32 = 0;
	for i in 0..32 {
		let a_bit: u32 = (a >> i) & 1;
		let b_bit: u32 = (b >> i) & 1;
		if carry_over == 0 {
			
			if a_bit & b_bit == 1 {
				carry_over = 1;
			} else if a_bit | b_bit != 0 {
				result = set_bit(result, i);
			}
		} else {
			if a_bit & b_bit == 1 {
				result = set_bit(result, i);
			} else if a_bit | b_bit == 0 {
				result = set_bit(result, i);
				carry_over = 0;	
			}
		}
	}
	result
}

fn main() {
	let array: [u32; 7] = [10, 42, 56, 689, 0, 42, 2456];
	for i in array.iter() {
		for j in array.iter() {
			println!("My result: {}, real results: {}", adder(*i, *j), *i + *j);
		}
	}
}
