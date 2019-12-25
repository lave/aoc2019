use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::cmp::max;
use std::cmp::Ordering;

mod common;

type Map = (Vec<Vec<u8>>, i32, i32);
type Asteroid = (i32, i32);


fn main() {
    let map = parse("10.input");
    let asteroids = find_asteroids(&map);
    //println!("{:?}", asteroids);

    //  part 1
    let (best, visible_count) = find_best_asteriod(&map, &asteroids);
    println!("{}", visible_count);

    //  part 2
    let asteroids = sort_asteriods(asteroids, &best);
    //println!("{:?}", best);
    //println!("{:?}", asteroids);

    let (x, y) = asteroids[200 - 1];
    println!("{}", x * 100 + y);
}


fn parse(filename: &str) -> Map {
    let f = File::open(filename).unwrap();
    let lines = BufReader::new(&f).lines();

    let map = lines.map(|line| line.unwrap().chars()
        .map(|c| if c == '#' { 1 } else { 0 })
        .collect::<Vec<u8>>()
    ).collect::<Vec<_>>();

    let w = map[0].len() as i32;
    let h = map.len() as i32;
    (map, w, h)
}


fn find_asteroids((map, w, h): &Map) -> Vec<Asteroid> {
    let mut result = Vec::new();
    for y in 0 .. *h {
        for x in 0 .. *w {
            if map[y as usize][x as usize] == 1 {
                result.push((x, y));
            }
        }
    }
    result
}

fn find_best_asteriod(map: &Map, asteroids: &Vec<Asteroid>) -> (Asteroid, u32) {
    asteroids.iter()
        .map(|a| (a.clone(), visible_from(map, a)))
        .max_by_key(|a| a.1)
        .unwrap()
}

fn visible_from(map: &Map, asteroid: &Asteroid) -> u32 {
    let (_, w, h) = map;
    let (x0, y0) = asteroid;

    //  2d array to mark locations obstructed by other asteroids
    let mut grid_data = vec![0; (w * h) as usize];
    let mut grid_rows = grid_data.chunks_mut(*w as usize).collect::<Vec<_>>();
    let obstructed = grid_rows.as_mut_slice();

    let mut count = 0;
    let maxd = max(max(*x0, *w - x0 - 1), max(*y0, *h - y0 - 1));
    for d in 1 ..= maxd {
        //println!("Depth {}", d);
        //  upper side
        if y0 - d >= 0 {
            for x in x0 - d .. x0 + d {
                count += trace(map, obstructed, asteroid, x, y0 - d);
            }
        }
        //  right side
        if x0 + d < *w {
            for y in y0 - d .. y0 + d {
                count += trace(map, obstructed, asteroid, x0 + d, y);
            }
        }
        //  lower side
        if y0 + d < *h {
            for x in (x0 - d .. x0 + d).rev() {
                count += trace(map, obstructed, asteroid, x + 1, y0 + d);
            }
        }
        //  left side
        if x0 - d >= 0 {
            for y in (y0 - d .. y0 + d).rev() {
                count += trace(map, obstructed, asteroid, x0 - d, y + 1);
            }
        }
    }
    count
}

fn trace(
    (map, w, h): &Map, obstructed: &mut[&mut[u8]],
    asteroid: &Asteroid, mut x: i32, mut y: i32) -> u32
{
    if x < 0 || x >= *w || y < 0 || y >= *h {
        0
    } else {
        //println!("{} {}", x, y);
        let (x0, y0) = asteroid;
        if map[y as usize][x as usize] == 0 || obstructed[y as usize][x as usize] == 1 {
            0
        } else {
            obstructed[y as usize][x as usize] = 1;
            let mut dx = x - x0;
            let mut dy = y - y0;
            let gcd = common::gcd(dx, dy);
            //println!("  {} {}: gcd {}", dx, dy, gcd);
            dx /= gcd;
            dy /= gcd;
            //println!("  {} {}", dx, dy);
            loop {
                x += dx;
                y += dy;
                if x < 0 || x >= *w || y < 0 || y >= *h {
                    break;
                } else {
                    //println!("  X {} {}", x, y);
                    obstructed[y as usize][x as usize] = 1;
                }
            }
            1
        }
    }
}

fn sort_asteriods(mut asteroids: Vec<Asteroid>, (x0, y0): &Asteroid) -> Vec<Asteroid> {
    asteroids.sort_unstable_by(|a, b| {
        let (xa_, ya_) = a;
        let (xb_, yb_) = b;
        let xa = xa_ - x0;
        let ya = ya_ - y0;
        let xb = xb_ - x0;
        let yb = yb_ - y0;

        if xa == 0 && xb == 0 {
            if ya * yb > 0 { ya.abs().cmp(&yb.abs()) }
            else if ya < 0 { Ordering::Less }
            else { Ordering::Greater }
        } else if xa == 0 {
            if ya < 0 || xb < 0 { Ordering::Less }
            else { Ordering::Greater }
        } else if xb == 0 {
            if yb < 0 || xa < 0 { Ordering::Greater }
            else { Ordering::Less }
        } else if xa * xb < 0 {//  if a and b are on different sides (left vs right)
            xb.cmp(&xa)     
        } else {
            let p1 = ya * xb;
            let p2 = xa * yb;

            if p1 < p2 { Ordering::Less }
            else if p1 > p2 { Ordering::Greater }
            else { (xa.abs() + ya.abs()).cmp(&(xb.abs() + yb.abs())) }
        }
    });

    let mut ranked = Vec::new();
    let mut prev = (0, 0);
    let mut depth = -1;
    for asteroid in asteroids {
        let (x, y) = asteroid;
        if x != *x0 || y != *y0 {
            let (x_, y_) = prev;
            depth = if (x - x0) * (y_ - y0) == (x_ - x0) * (y - y0) { depth + 1 } else { 0 };
            ranked.push((x, y, depth));
            prev = asteroid.clone();
        }
    }

    ranked.sort_by(|a, b| { a.2.cmp(&b.2) });

    ranked.into_iter().map(|(x, y, _)| (x, y)).collect()
}

