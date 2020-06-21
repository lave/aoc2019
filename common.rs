#[allow(dead_code)]
pub fn split_at_pattern<'a, 'b>(s: &'a str, p: &'b str) -> (&'a str, &'a str) {
    let pos = s.find(p).unwrap();
    let (r, s) = s.split_at(pos);
    let s = s.trim_start_matches(p);
    (r, s)
}

#[allow(dead_code)]
pub fn maybe_trim_prefix<'a, 'b>(s: &'a str, p: &'b str) -> Option<&'a str> {
    if s.starts_with(p) {
        Some(&s[p.len() ..])
    } else {
        None
    }
}

#[allow(dead_code)]
pub fn to_i32(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
}

#[allow(dead_code)]
pub fn to_u32(s: &str) -> u32 {
    s.parse::<u32>().unwrap()
}

#[allow(dead_code)]
pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    a = a.abs();
    b = b.abs();
    if a < b {
        std::mem::swap(&mut a, &mut b);
    }
    if b == 0 {
        a
    } else {
        let mut r = a % b;
        while r > 0 {
            a = b;
            b = r;
            r = a % b;
        }
        b
    }
}

