use crate::adder;

fn generate_mask(value: u32) -> u32 {
    return if value == 1 {4294967295} else {0};
}

pub fn multiplier(a: u32, b: u32) -> u32 {
    let mut res: u32 = 0;
    for i in 0..32 {
        let mask: u32 = generate_mask(b >> i & 1);
        res = adder(res, (a & mask) << i);
    }
    res
}
