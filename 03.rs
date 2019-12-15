use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::cmp;


#[derive(Debug, Copy, Clone)]
struct Point(i32, i32);

#[derive(Debug)]
struct Line(Point, Point, i32);

type Track = Vec<Line>;



fn main() {
    let (track1, track2) = parse("03.input");
    //println!("{:?}", track1);
    //println!("{:?}", track2);

    //  part 1
    let intersections = find_intersections(&track1, &track2);
    //println!("{:?}", intersections);
    let dist1 = intersections.iter()
        .map(| (Point(x, y), _d) | { x.abs() + y.abs() })
        .min().unwrap();
    println!("{}", dist1);

    //  part 2
    let dist2 = intersections.iter()
        .map(| p | { p.1 })
        .min().unwrap();
    println!("{}", dist2);
}


fn parse(filename: &str) -> (Track, Track) {
    let f = File::open(filename).unwrap();
    let mut lines = BufReader::new(&f).lines();
    let line1 = lines.next().unwrap().unwrap();
    let line2 = lines.next().unwrap().unwrap();
    return (parse_track(line1), parse_track(line2));
}

fn parse_track(line: String) -> Track {
    line.split(',').scan(
        (Point(0, 0), 0),
        | (p, d), e | {
            let d0 = d.clone();
            let (d_, l_) = e.split_at(1);
            let l = l_.parse::<i32>().unwrap();
            let p_ = p.clone();
            let Point(x, y) = p;
            match d_ {
                "L" => *x -= l,
                "R" => *x += l,
                "D" => *y -= l,
                "U" => *y += l,
                _ => panic!()
            };
            *d += l;
            Some(Line(p_, *p, d0))
        }
    ).collect()
}

fn find_intersections(track1: &Track, track2: &Track) -> Vec<(Point, i32)> {
    let mut points = Vec::new();

    for Line(Point(x11, y11), Point(x12, y12), d1) in track1 {
        for Line(Point(x21, y21), Point(x22, y22), d2) in track2 {
            let h1 = y11 == y12;
            let h2 = y21 == y22;

            if h1 ^ h2 {
                let x  = if h1 { x21 } else { x11 };
                let x1 = if h1 { x11 } else { x21 };
                let x2 = if h1 { x12 } else { x22 };
                let y  = if h1 { y11 } else { y21 };
                let y1 = if h1 { y21 } else { y11 };
                let y2 = if h1 { y22 } else { y12 };

                let x1_ = cmp::min(x1, x2);
                let x2_ = cmp::max(x1, x2);
                let y1_ = cmp::min(y1, y2);
                let y2_ = cmp::max(y1, y2);

                if x1_ <= x && x <= x2_ && y1_ <= y && y <= y2_ {
                    let d1_ = if h1 { d1 + (x11 - x).abs() } else { d1 + (y11 - y).abs() };
                    let d2_ = if h1 { d2 + (y21 - y).abs() } else { d2 + (x21 - x).abs() };
                    
                    points.push((Point(*x, *y), d1_ + d2_));
                }
            }
        }
    }
    points
}


/*

fn process(codes: &Vec<u32>, noun: u32, verb: u32) -> u32 {
    let mut codes = codes.to_vec();
    codes[1] = noun;
    codes[2] = verb;

    let mut i = 0;
    loop {
        let opcode = codes[i];
        let a1 = codes[i + 1] as usize;
        let a2 = codes[i + 2] as usize;
        let a3 = codes[i + 3] as usize;
        i += 4;

        match opcode {
            1 => codes[a3] = codes[a1] + codes[a2],
            2 => codes[a3] = codes[a1] * codes[a2],
            99 => break,
            _ => panic!("unknown opcode {}", opcode)
        }
    }
    codes[0]
}


fn find(codes: &Vec<u32>, target: u32) -> Option<(u32, u32)> {
    for noun in 0 .. 99 {
        for verb in 0 .. 99 {
            if process(codes, noun, verb) == target {
                return Some((noun, verb));
            }
        }
    }
    None
}
*/
