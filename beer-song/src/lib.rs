
pub fn bottles(n: u32) -> String {
    if n == 0 {
        format!("no more bottles")
    } else if n == 1 {
        format!("1 bottle")
    } else {
        format!("{} bottles", n)
    }
}

pub fn row2(n: u32) -> String {
    if n == 0 {
        format!("Go to the store and buy some more")
    } else if n == 1 {
        format!("Take it down and pass it around")
    } else {
        format!("Take one down and pass it around")
    }
}

pub fn verse(n: u32) -> String {
    let nn: u32;
    if n == 0 {
        nn = 99;
    } else {
        nn = n -1;
    }
    let mut r = format!("{0} of beer on the wall, {0} of beer.\n{1}, {2} of beer on the wall.\n", bottles(n),row2(n),bottles(nn));
    r.get_mut(..1).map(str::make_ascii_uppercase);
    r
}

pub fn sing(start: u32, end: u32) -> String {
    let mut v: Vec<String> = Vec::new();
    for i in end..start+1 {
        v.push(verse(i))
    }
    v.reverse();
    let s = v.join("\n");
    s
}
