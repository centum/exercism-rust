pub fn is_armstrong_number(num: u32) -> bool {
    let mut m: Vec<u32> = Vec::new();
    let mut c = num;
    while c != 0 {
        m.push(c %10);
        c /=10;
    }
    num == m.iter().fold(0, |s, i| s + i.pow(m.len() as u32))
}
