pub fn map(x: u16, y: u16) -> f64 {
    let mut interleaved: u32 = 0;
    let mut shift = 0;

    for i in 0..16 {
        let bit_x = (x >> i) & 1;
        interleaved |= (bit_x as u32) << shift;
        shift += 1;
		let bit_y = (y >> i) & 1;
		interleaved |= (bit_y as u32) << shift;
        shift += 1;
    }
    interleaved as f64 / 4294967295.0
}
