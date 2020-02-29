pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut res: u32 = 0;

    for (i, dd) in factors.iter().enumerate() {
        let d = *dd;
        if d <= 0 {
            continue
        }

        let m = (limit-1)/d * d;

        'loop_b: for n in (d..=m).rev().step_by(d as usize) {
            for k in 0..i {
                if n % factors[k] == 0 {
                    continue 'loop_b;
                }
            }
            res += n;
        }
    }

    res
}
