#[allow(dead_code)]
pub fn split_at_pattern<'a, 'b>(s: &'a str, p: &'b str) -> (&'a str, &'a str) {
    let pos = s.find(p).unwrap();
    let (r, s) = s.split_at(pos);
    let s = s.trim_start_matches(p);
    (r, s)
}

#[allow(dead_code)]
pub fn to_i32(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
}

#[allow(dead_code)]
pub fn to_u32(s: &str) -> u32 {
    s.parse::<u32>().unwrap()
}

