fn set_bit(nb: u32, bit_nb: u32) -> u32 {
	return (1 << bit_nb) | nb;
}

fn adder(a: u32, b: u32) -> u32 {
	println!("{:032b}", a);
	// println!("{:032b}", b);
	// println!("{:032b}", a + b);
	// println!("{:032b}", (a | b) << 1);

	let mut result: u32 = 0;
	let mut carry_over: u32 = 0;
	for i in 0..32 {
		let a_bit: u32 = (a >> i) & 1;
		let b_bit: u32 = (b >> i) & 1;

		if a_bit | b_bit | carry_over == 1 {
			result = 1
			if carry_over >= 1 {
				carry_over = 0
			}
		} else if a_bit & b_bit & carry_over > 1 {
			
		}
		// if a_bit | b_bit | carry_over - 0 {
		// 	result = set_bit(result, 31 - i);
		// } else if a_bit & b_bit != 0 {
		// 	carry_over = 1;
		// 	if carry_over != 0 {
		// 		result = set_bit(result, i);	
		// 	}
		// }
	}
	result
}

fn main() {
	println!("{}", adder(15, 5));
}
