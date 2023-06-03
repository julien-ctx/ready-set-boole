use crate::adder;

fn add_combinations(vec: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, permutation: &mut Vec<i32>, len: i32) {
	if len == vec.len() as i32 {
		let mut elem: Vec<i32> = Vec::new();
		for (index, perm) in permutation.iter().enumerate() {
			if *perm == 1 {
				elem.push(vec[index]);
			}
		}
		res.push(elem);
	} else {
		for i in 0..2 {
			permutation[len as usize] = i;
			add_combinations(vec, res, permutation, adder(len as u32, 1) as i32);
		}
	}
}


pub fn powerset(set: &[i32]) -> Vec<Vec<i32>> {
	let mut vec: Vec<i32> = set.to_vec();
	let mut res: Vec<Vec<i32>> = Vec::new();
	let mut permutation: Vec<i32> = Vec::new();
	for _ in &vec {
		permutation.push(0);
	}

	add_combinations(&mut vec, &mut res, &mut permutation, 0);

	res.sort_by(|a, b| {
		let a_len = a.len();
		let b_len = b.len();
		if a_len != b_len {
			return a_len.cmp(&b_len);
		} else {
			return a.cmp(&b);
		}
	});
	res
}
