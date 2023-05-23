pub fn adder(a: u32, b: u32) -> u32 {
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


fn generate_mask(value: u32) -> u32 {
    return if value == 1 {4294967295} else {0};
}

pub fn multiplier(a: u32, b: u32) -> u32 {
    // let mut numbers: Vec<u32> = Vec::new();
    let mut res: u32 = 0;
    for i in 0..32 {
        let mask: u32 = generate_mask(b >> i & 1);
        res = adder(res, (a & mask) << i);
    }
    res
}
