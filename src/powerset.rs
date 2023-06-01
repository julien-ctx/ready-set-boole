

pub fn powerset(set: &[i32]) -> Vec<Vec<i32>> {
	let mut vec: Vec<i32> = set.to_vec();
	let mut res: Vec<Vec<i32>> = Vec::new();

	res.push(Vec::new());

	

	res.push(vec);
	res
}
