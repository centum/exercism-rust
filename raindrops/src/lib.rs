pub fn raindrops(n: u32) -> String {
    let mut res = String::new();
    if n % 3 == 0 {
        res.push_str(&"Pling".to_string())
    }
    if n%5 == 0 {
        res.push_str(&"Plang".to_string())
	}
	if n%7 == 0 {
        res.push_str(&"Plong".to_string())
	}
	if res == "" {
        res.push_str(&n.to_string())
	}
    res
}
