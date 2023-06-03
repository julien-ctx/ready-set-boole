use crate::adder;

pub fn gray_code(n: u32) -> u32 {
	let mut res: u32 = 0;
	for i in 0..31 {
		// XOR result
		let binary: u32 = (n >> i & 1) ^ (n >> adder(i, 1) & 1);
		res |= binary << i;
	}
	res
}
