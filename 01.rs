use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {
    let f = File::open("01.input").unwrap();
    let file = BufReader::new(&f);

    //  part 1
    let mut f0 = 0;
    let mut f1 = 0;
    for line in file.lines() {
        let l = line.unwrap();
        let n = l.parse::<i32>().unwrap();
        f0 += fuel(n);
        f1 += fuel_(n);
    }

    println!("{}", f0);

    //  part 2
    println!("{}", f1);
}

fn fuel(m: i32) -> i32 {
    m / 3 - 2
}

fn fuel_(m: i32) -> i32 {
    let mut f = fuel(m);
    let mut f_ = f;
    while f > 0 {
        f = fuel(f);
        if f > 0 {
            f_ += f;
        }
    }
    f_
}
