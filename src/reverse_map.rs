pub fn reverse_map(n: f64) -> (u16, u16) {
	let real_nb = (n * 4294967295.0) as u32;
	let mut x: u16 = 0;
	let mut y: u16 = 0;
	let mut shift: u32 = 0;
	for i in 0..16 {
		let bit = (real_nb >> shift) & 1;
		x |= (bit as u16) << i;
		shift += 1;
		let bit = (real_nb >> shift) & 1;
		y |= (bit as u16) << i;
		shift += 1;
	}
	(x, y)
}
