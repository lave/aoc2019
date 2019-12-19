use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;


fn main() {
    let data = parse("08.input");
    let n = 25 * 6;
    let bitmaps = data.chunks(n).collect::<Vec<_>>();
    //println!("{:?}", bitmaps);

    //  part 1
    let counts = bitmaps.iter()
        .map(| b | get_counts(&b))
        .min_by_key(| c | c[0])
        .unwrap();
    println!("{:?}", counts[1] * counts[2]);

    //  part 2
    let bitmap = bitmaps.iter()
        .fold(vec![2i8; n], | a, b | underlay(a, &b));
    print(&bitmap, 25);
}


fn parse(filename: &str) -> Vec<i8> {
    let f = File::open(filename).unwrap();
    let mut lines = BufReader::new(&f).lines();
    let line = lines.next().unwrap().unwrap();
    line.chars()
        .map(|c| (c as i8) - 0x30)
        .collect()
}

fn get_counts(ns: &[i8]) -> [i32; 10] {
    let mut counts = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for i in ns.iter() {
        counts[*i as usize] += 1;
    }
    counts
}

fn underlay(mut a: Vec<i8>, b: &[i8]) -> Vec<i8> {
    for i in 0 .. a.len() {
        if a[i] == 2 {
            a[i] = b[i];
        }
    }
    a
}

fn print(v: &Vec<i8>, n: usize) {
    for l in v.chunks(n) {
        println!("{}", l.iter()
            .map(| x | if *x == 0 { ' ' } else { '*' })
            .collect::<String>());
    }
}
