use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

mod common;
use common::*;


#[derive(Debug)]
enum Shuffle {
    New,
    Cut(i32),
    Inc(u32)
}

type Matrix = [i128; 2];


fn main() {
    let shuffles = parse("22.input");
    //println!("{:?}", shuffles);

    //  part 1
    let pos = trace_card(&shuffles, 10007, 2019);
    println!("{}", pos);

    //  part 2
    let s = 119315717514047i128;
    let m = power_matrix(&build_matrix(&shuffles, s), s, 101741582076661);
    //println!("{:?}", m);

    //  find out what item is at place 2020
    let num = mod_divide(2020 - m[1] + s, m[0], s);
    //  test that found element will indeed be at place 2020
    let pos = (m[0] * num + m[1]) % s;
    assert!(pos == 2020);
    println!("{}", num);
}


fn parse(filename: &str) -> Vec<Shuffle> {
    let f = File::open(filename).unwrap();
    let lines = BufReader::new(&f).lines();
    lines.map(|line| parse_shuffle(&line.unwrap())).collect()
}

fn parse_shuffle(line: &str) -> Shuffle {
    if line == "deal into new stack" {
        Shuffle::New
    } else if let Some(n) = maybe_trim_prefix(line, "cut ") {
        Shuffle::Cut(to_i32(n))
    } else if let Some(n) = maybe_trim_prefix(line, "deal with increment ") {
        Shuffle::Inc(to_u32(n))
    } else {
        panic!("unknown shuffle")
    }
}


fn trace_card(shuffles: &Vec<Shuffle>, s: i64, mut pos: i64) -> i64 {
    for shuffle in shuffles {
        pos = match shuffle {
            Shuffle::New => s - pos - 1,
            Shuffle::Cut(n) => (pos + s - *n as i64) % s,
            Shuffle::Inc(n) => (pos * *n as i64) % s
        };
    }
    pos
}


fn mutliply(s: i128, m1: &Matrix, m2: &Matrix) -> Matrix {
    [(m1[0] * m2[0]         + s) % s,
     (m1[1] * m2[0] + m2[1] + s) % s]
}


fn build_matrix(shuffles: &Vec<Shuffle>, s: i128) -> Matrix {
    let mut m = [1, 0];
    for shuffle in shuffles {
        let m_ = match shuffle {
            Shuffle::New => [-1, (s - 1) as i128],
            Shuffle::Cut(n) => [1, -n as i128],
            Shuffle::Inc(n) => [*n as i128, 0]
        };
        m = mutliply(s, &m, &m_);
    }
    m
}

fn power_matrix(m: &Matrix, s: i128, mut p: i128) -> Matrix {
    let mut powers = Vec::new();
    let mut m_ = m.clone();
    for _i in 0 .. 64 {
        powers.push(m_.clone());
        m_ = mutliply(s, &m_, &m_);
    }

    let mut m_ = [1, 0];
    let mut i = 0;
    while p > 0 {
        if (p & 1) == 1 {
            m_ = mutliply(s, &m_, &powers[i]);
        }
        i += 1;
        p >>= 1;
    }
    m_
}


//  function to compute a/b under modulo m
fn mod_divide(a: i128, b: i128, m: i128) -> i128 {
    let a = a % m;
    let inv = mod_inverse(b, m).unwrap();
    (inv * a) % m
}

//  function to find modulo inverse of b
fn mod_inverse(b: i128, m: i128) -> Option<i128> {
    let (gcd, x, _) = gcd_extended(b, m);

    if gcd != 1 {
        // return None if b and m are not co-prime
        None
    } else {
        // m is added to handle negative x
        Some((x % m + m) % m)
    }
}

fn gcd_extended(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (gcd, x, y) = gcd_extended(b % a, a);
        (gcd, y - (b / a) * x, x)
    }
}
