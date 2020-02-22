pub fn build_proverb(list: &[&str]) -> String {
    if list.len() == 0 {
        return String::new();
    }
    let mut v:Vec<String>  = Vec::new();
    for i in 1..list.len() {
        v.push(format!("For want of a {} the {} was lost.", list[i-1], list[i]));
    }
    v.push(format!("And all for the want of a {}.", list[0]));
    v.join("\n")
}
