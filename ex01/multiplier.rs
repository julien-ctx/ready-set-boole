fn set_bit(nb: u32, bit_nb: u32) -> u32 {
	return (1 << bit_nb) | nb;
}

fn multiplier(a: u32, b: u32) -> u32 {

}

fn main() {
	let array: [u32; 7] = [17, 44, 6, 69, 0, 6, 245];
	for i in array.iter() {
		for j in array.iter() {
			println!("My result: {}, real results: {}", adder(*i, *j), *i + *j);
		}
	}
}
