use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;


fn main() {
    let (low, hi) = parse("04.input");
    //println!("{:?} {:?}", low, hi);

    //  part 1
    let (c1, c2) = count_passwords(low, hi);
    println!("{}", c1);

    //  part 2
    println!("{}", c2);
}


fn parse(filename: &str) -> (u32, u32) {
    let f = File::open(filename).unwrap();
    let mut lines = BufReader::new(&f).lines();
    let line = lines.next().unwrap().unwrap();
    let mut parts = line.split('-');
    return (parts.next().unwrap().parse::<u32>().unwrap(),
            parts.next().unwrap().parse::<u32>().unwrap());
}


fn count_passwords(low: u32, hi: u32) -> (u32, u32) {
    let mut ds = digits6(low);
    let mut c1 = 0;
    let mut c2 = 0;

    for _i in low..=hi {
        if is_good_1(ds) {
            //println!("OK1: {:?}", ds);
            c1 += 1;

            if is_good_2(ds) {
                //println!("OK2: {:?}", ds);
                c2 += 1;
            }
        }

        for j in 0..6 {
            ds[j] += 1;
            if ds[j] < 10 {
                break;
            }
            ds[j] = 0;
        }
    }
    (c1, c2)
}

fn digits6(mut x: u32) -> [u8; 6] {
    let mut ds: [u8; 6] = [0; 6];
    for i in 0..6 {
        ds[i] = (x % 10) as u8;
        x /= 10;
    }
    ds
}

fn is_good_1(ds: [u8; 6]) -> bool {
    let mut has_dup = false;
    let mut no_inc = true;

    let mut d = ds[0];
    for j in 1..6 {
        let d_ = ds[j];
        has_dup |= d == d_;
        if d < d_ {
            no_inc = false;
            break;
        }
        d = d_;
    }

    has_dup && no_inc
}

fn is_good_2(ds: [u8; 6]) -> bool {
    let mut d = ds[0];
    let mut l = 1;

    for j in 1..6 {
        let d_ = ds[j];
        if d == d_ {
            l += 1;
        } else {
            if l == 2 {
                break;
            }
            l = 1;
            d = d_;
        }
    }
    l == 2
}
