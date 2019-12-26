use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashSet;

type Map = u32;


fn main() {
    let (map, w, h) = parse("24.input");
    //print_map(map, w, h);

    //  part 1
    let repeating = find_loop(map, w, h);
    println!("{}", repeating);

    //  part 2
    let maps = vec![map];
    let maps_ = live_rec(maps, w, h, 200);

    let ones: u32 = maps_.iter().map(|m| m.count_ones()).sum();
    println!("{:?}", ones);
}


fn parse(filename: &str) -> (Map, u32, u32) {
    let f = File::open(filename).unwrap();
    let lines = BufReader::new(&f).lines();

    let mut map = 0;
    let mut w = 0;
    let mut h = 0;
    let mut i = 0;
    for line in lines {
        for c in line.unwrap().chars() {
            if c == '#' {
                map |= 1 << i;
            }
            i += 1;
            if h == 0 { w += 1; }
        }
        h += 1;
    }
    (map, w, h)
}


fn print_map(map: Map, w: u32, h: u32) {
    let n = w * h;
    for i in 0 .. n {
        let c = if map & (1 << i) == 0 { '.' } else { '#' };
        print!("{}", c);
        if (i + 1) % w == 0 { println!(); }
    }
    println!();
}


fn bit(v: u32) -> u32 {
    if v == 0 { 0 } else { 1 }
}

fn live(map: Map, w: u32, h: u32) -> Map {
    let n = w * h;
    let mut m_ = 0;
    for i in 0 .. n {
        let m = 1 << i;
        let v = bit(map & m);
        let x = i % w;
        let y = i / w;
        let l = if x == 0     { 0 } else { bit(map & (m >> 1)) };
        let r = if x == w - 1 { 0 } else { bit(map & (m << 1)) };
        let u = if y == 0     { 0 } else { bit(map & (m >> 5)) };
        let d = if y == h - 1 { 0 } else { bit(map & (m << 5)) };
        let sum = l + r + u + d;

        let v_ = if v == 1 {
            if sum == 1 { 1 } else { 0 }
        } else {
            if sum == 1 || sum == 2 { 1 } else { 0 }
        };
        if v_ == 1 { m_ |= 1 << i; }
    }
    m_
}


fn find_loop(mut map: Map, w: u32, h: u32) -> Map {
    let mut maps = HashSet::new();
    loop {
        map = live(map, w, h);
        //print_map(map, w, h);
        if maps.contains(&map) { break; }
        maps.insert(map);
    }
    map
}


fn live_rec(mut maps: Vec<Map>, w: u32, h: u32, n: u32) -> Vec<Map> {
    for _ in 0 .. n {
        maps = live_rec_(maps, w, h);
    }
    maps
}

fn live_rec_(maps: Vec<Map>, w: u32, h: u32) -> Vec<Map> {
    let mut maps_ = Vec::new();
    let mut outer = 0;
    let mut map = 0;
    for i in 0 .. maps.len() + 2 {
        let inner = if i < maps.len() { maps[i] } else { 0 };

        let map_ = if map == 0 && outer == 0 && inner == 0 { 0 } else { live3(map, outer, inner, w, h) };
        if map_ != 0 || !maps_.is_empty() { maps_.push(map_); }

        outer = map;
        map = inner;
    }

    while *maps_.last().unwrap() == 0 {
        maps_.pop();
    }
    maps_
}

fn live3(map: Map, outer: Map, inner: Map, w: u32, h: u32) -> Map {

    fn sum_left(map: Map, w: u32, h: u32) -> u32 {
        let mut s = 0;
        let mut mask = 1;
        for _ in 0 .. h {
            if (map & mask) != 0 { s += 1; }
            mask <<= w;
        }
        s
    }

    fn sum_right(map: Map, w: u32, h: u32) -> u32 {
        let mut s = 0;
        let mut mask = 1 << (w - 1);
        for _ in 0 .. h {
            if (map & mask) != 0 { s += 1; }
            mask <<= w;
        }
        s
    }

    fn sum_up(map: Map, w: u32, _h: u32) -> u32 {
        let mut s = 0;
        let mut mask = 1;
        for _ in 0 .. w {
            if (map & mask) != 0 { s += 1; }
            mask <<= 1;
        }
        s
    }

    fn sum_down(map: Map, w: u32, h: u32) -> u32 {
        let mut s = 0;
        let mut mask = 1 << (w * h - 1);
        for _ in 0 .. w {
            if (map & mask) != 0 { s += 1; }
            mask >>= 1;
        }
        s
    }

    let n = w * h;
    let mut m_ = 0;

    let n2 = n / 2;
    let xc = w / 2;
    let yc = h / 2;

    let outer_l = bit(outer & (1 << (n2 - 1)));
    let outer_r = bit(outer & (1 << (n2 + 1)));
    let outer_u = bit(outer & (1 << (n2 - w)));
    let outer_d = bit(outer & (1 << (n2 + w)));

    for i in 0 .. n {
        let x = i % w;
        let y = i / w;
        if x != xc || y != yc { 
            let m = 1 << i;
            let v = bit(map & m);

            let l =
                if x == 0 {
                    outer_l
                } else if x == xc + 1 && y == yc {
                    sum_right(inner, w, h)
                } else {
                    bit(map & (m >> 1))
                };

            let r =
                if x == w - 1 {
                    outer_r
                } else if x == xc - 1 && y == yc {
                    sum_left(inner, w, h)
                } else {
                    bit(map & (m << 1))
                };

            let u =
                if y == 0 {
                    outer_u
                } else if y == yc + 1 && x == xc {
                    sum_down(inner, w, h)
                } else {
                    bit(map & (m >> w))
                };

            let d =
                if y == h - 1 {
                    outer_d
                } else if y == yc - 1 && x == xc {
                    sum_up(inner, w, h)
                } else {
                    bit(map & (m << w))
                };

            let sum = l + r + u + d;

            let v_ = if v == 1 {
                if sum == 1 { 1 } else { 0 }
            } else {
                if sum == 1 || sum == 2 { 1 } else { 0 }
            };
            if v_ == 1 { m_ |= 1 << i; }
        }
    }

    m_
}
