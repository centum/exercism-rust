pub fn factors(n: u64) -> Vec<u64> {
    let mut v: Vec<u64> = Vec::new();
    let mut m = n;
    let mut p: u64 = 2;
    loop {
        if m % p == 0 {
            v.push(p);
            if m == p {
                return v;
            }
            m /= p;
        } else {
            p+=1;
        }
    }
}
