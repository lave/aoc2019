use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

type Data = Vec<i8>;


fn main() {
    let data = parse("16.input");
    //println!("{:?}", data);

    //  part 1
    let decoded = decode(&data, 100);
    println!("{}", to_number(&decoded[0..8]));

    //  part 2
    let pos = to_number(&data[0 .. 7]);
    let decoded = decode_tail(&data, 10000, pos, 100);
    println!("{}", to_number(&decoded[0..8]));
}


fn parse(filename: &str) -> Data {
    let f = File::open(filename).unwrap();
    let mut lines = BufReader::new(&f).lines();
    let line = lines.next().unwrap().unwrap();
    return line.chars().map(|c| c.to_digit(10).unwrap() as i8).collect();
}

fn decode(data: &Data, n: u32) -> Data {
    assert!(n > 0);
    let mut d = decode1(data);
    for _ in 1 .. n {
        let d_ = decode1(&d);
        d = d_;
    }
    d
}

fn decode1(data: &Data) -> Data {
    let mut data_ = vec![0; data.len()];

    for i in 0 .. data.len() {
        let mut sum = 0i32;
        let mut k = 1;
        let mut v = 0;
        for j in 0 .. data.len() {
            if k > i { k = 0; v = (v + 1) & 3; }
            k += 1;
            let p = if v == 3 { -1 } else { v & 1 };
            sum += (data[j] * p) as i32;
        }
        let d = sum.abs() % 10;
        data_[i] = d as i8;
    }
    data_
}


fn to_number(digits: &[i8]) -> usize {
    let mut n = 0;
    for d in digits {
        n = n * 10 + *d as usize;
    }
    n
}

fn decode_tail(data: &Data, repeat: u32, pos: usize, n: u32) -> Data {
    let l = data.len();

    assert!(pos > l / 2, "current algorithm only works in assumption that tails is shorter then half of the array");

    let l_ = l * (repeat as usize) - pos;
    let mut data_ = vec![0; l_];

    let mut j = pos % data.len();
    for i in 0 .. l_ {
        data_[i] = data[j];
        j += 1;
        if j == l { j = 0 };
    }

    let mut d = data_;
    for _ in 0 .. n {
        let d_ = decode_tail1(&d);
        d = d_;
    }
    d
}

fn decode_tail1(data: &Data) -> Data {
    let mut data_ = vec![0; data.len()];

    let li = data.len() - 1;
    data_[li] = data[li];
    for i in (0 .. li).rev() {
        data_[i] = (data[i] + data_[i + 1]) % 10;
    }
    data_
}
