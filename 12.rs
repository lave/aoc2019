use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

mod common;
use common::*;

type Coord = (i32, i32, i32);
type Speed = (i32, i32, i32);
type Body = (Coord, Speed);



fn main() {
    let bodies = parse("12.input");
    //println!("{:?}", bodies);

    //  part 1
    let bodies1 = simulate(&bodies, 1000);
    //println!("{:?}", bodies1);
    let energy = bodies1.iter().map(|b| energy(b)).sum::<i32>();
    println!("{:?}", energy);

    //  part 2
    simulate_loop(&bodies);
}


fn parse(filename: &str) -> Vec<Body> {
    let f = File::open(filename).unwrap();
    let lines = BufReader::new(&f).lines();
    lines.map(|line| parse_body(&line.unwrap())).collect()
}

fn parse_body(line: &str) -> Body {
    let line = line.trim_start_matches("<x=");
    let (x, line) = split_at_pattern(line, ", y=");
    let (y, line) = split_at_pattern(line, ", z=");
    let (z, _) = split_at_pattern(line, ">");

    ((to_i32(x), to_i32(y), to_i32(z)), (0, 0, 0))
}

fn simulate(bodies: &Vec<Body>, n: u32) -> Vec<Body> {
    let mut bodies = bodies.clone();
    for _ in 0 .. n {
        simulate1(&mut bodies);
    }
    bodies
}

fn simulate1(bodies: &mut Vec<Body>) {
    //  update speeds
    let m = bodies.len();
    for b1 in 0 .. m {
        for b2 in 0 .. b1 {
            let ((x1, y1, z1), _) = &bodies[b1];
            let ((x2, y2, z2), _) = &bodies[b2];

            let dx = if x1 < x2 { -1 } else if x1 > x2 { 1 } else { 0 };
            let dy = if y1 < y2 { -1 } else if y1 > y2 { 1 } else { 0 };
            let dz = if z1 < z2 { -1 } else if z1 > z2 { 1 } else { 0 };

            let (_, (x, y, z)) = &mut bodies[b1];
            *x -= dx;
            *y -= dy;
            *z -= dz;

            let (_, (x, y, z)) = &mut bodies[b2];
            *x += dx;
            *y += dy;
            *z += dz;
        }
    }

    //  update positions
    for b in 0 .. m {
        let ((x, y, z), (x_, y_, z_)) = &mut bodies[b];
        *x += *x_;
        *y += *y_;
        *z += *z_;
    }
}

fn energy(((x, y, z), (x_, y_, z_)): &Body) -> i32 {
    (x.abs() + y.abs() + z.abs()) * (x_.abs() + y_.abs() + z_.abs())
}


fn simulate_loop(bodies: &Vec<Body>) {
    let mut bodies = bodies.clone();
    let mut bodies2 = bodies.clone();

    let mut n = 0u64;
    loop {
        simulate1(&mut bodies);
        simulate1(&mut bodies2);
        simulate1(&mut bodies2);

        n += 1;
        if n % 1000000 == 0 {
            println!("{:?}", n);
        }

        if bodies == bodies2 { break; }
    }

    println!("{:?}", n);
    println!("{:?}", bodies);
    println!("{:?}", bodies2);
}
