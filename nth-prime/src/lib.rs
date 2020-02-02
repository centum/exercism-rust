pub fn nth(n: u32) -> u32 {
    let mut n = n + 1;
    if n == 1 {
		return 2;
	}

    let mut p: u32 = 1;

    'loop0: while n > 1 {
        p += 2;
        for i in (3..p).step_by(2) {
			if p%i == 0 {
				continue 'loop0
			}
		}
        n-=1;
    }

    p
}
