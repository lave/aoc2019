mod intcode;

type Coord = (i32, i32);
type Map = Vec<bool>;


fn main() {
    let program = intcode::read("19.input");

    //  part 1
    let n = 50;
    let map = run(&program, 0, 0, n - 1, n - 1);
    //print_map(&map, n);
    println!("{}", map.iter().filter(|&&v| v).count());

    //  part 2
    let (p1, p2) = get_bounds(&map, n);
    //println!("{:?} - {:?}", p1, p2);

    let (p1, p2) = find(&program, p1, p2, 99);
    //println!("{:?} - {:?}", p1, p2);
    println!("{}", p2.0 * 10000 + p1.1);
}


fn is_affected(program: &intcode::Program, x: i32, y: i32) -> bool {
    intcode::run(program, vec![x as intcode::Word, y as intcode::Word])[0] == 1
}

fn run(program: &intcode::Program, x1: i32, y1: i32, x2: i32, y2: i32) -> Map {
    let mut output = Vec::new();
    for y in y1 ..= y2 {
        for x in x1 ..= x2 {
            output.push(is_affected(program, x, y));
        }
    }
    output
}

#[allow(dead_code)]
fn print_map(map: &Map, w: i32) {
    for line in map.chunks(w as usize) {
        println!("{}", line.iter().map(|v| if *v { '#' } else { '.' }).collect::<String>());
    }
}


fn get_bounds(map: &Map, n: i32) -> (Coord, Coord) {
    let mut p10 = None;
    let mut p11 = None;
    let mut p21 = None;
    let mut p20 = None;

    let mut p = (n - 1, 0);
    for i in 0 .. n * 2 - 1 {
        let x = if i < n { n - 1 } else { n - 1 - (i - n) };
        let y = if i < n { i } else { n };
        let c = (y * n + x) as usize;
        if p10.is_none() && map[c] {
            p10 = Some(p);
            p11 = Some((x, y));
        } else if p10.is_some() && !map[c] {
            p21 = Some(p);
            p20 = Some((x, y));
            break;
        }
        p = (x, y);
    }

    let (x10, y10) = p10.unwrap();
    let (x11, y11) = p11.unwrap();
    let (x21, y21) = p21.unwrap();
    let (x20, y20) = p20.unwrap();
    ((x10 + x11, y10 + y11), (x21 + x20, y21 + y20))
}


fn find (program: &intcode::Program, (x10, y10): Coord, (x20, y20): Coord, n: i32) -> (Coord, Coord) {

    fn adjust(program: &intcode::Program, mut x: i32, mut y: i32, target: bool, clockwise: bool) -> Coord {
        //  parameter to track which coord to touch this time (Brezenham's-like algorithm)
        let x0 = x;
        let y0 = y;
        let mut p = x0 - y0;
        //  previous values
        let mut x_;
        let mut y_;
        loop {
            x_ = x;
            y_ = y;
            if p > 0 {
                p -= y0; y += if clockwise { 1 } else { -1 };
            } else {
                p += x0; x -= if clockwise { 1 } else { -1 };
            }
            if is_affected(program, x, y) == target { break; }
        }
        //println!("{} {} is {}, {} {} is {}", x_, y_, !target, x, y, target);

        //  always return points which are "affected"
        if target { (x, y) } else { (x_, y_) }
    }

    let mut x1 = x10;
    let mut y1 = y10;
    let mut x2 = x20;
    let mut y2 = y20;
    let mut changed = true;
    while changed {
        changed = false;
        let d = y2 * x1 - x2 * y1;
        let x1_ = n * x1 * (x2 + y2) / d;
        let y1_ = n * y1 * (x2 + y2) / d;
        let x2_ = n * x2 * (x1 + y1) / d;
        let y2_ = n * y2 * (x1 + y1) / d;

        if is_affected(program, x1_, y1_) {
            let (x, y) = adjust(program, x1_, y1_, false, false);
            if (x, y) != (x1, y1) { x1 = x; y1 = y; changed = true; }
        } else {
            let (x, y) = adjust(program, x1_, y1_, true, true);
            if (x, y) != (x1, y1) { x1 = x; y1 = y; changed = true; }
        }

        if is_affected(program, x2_, y2_) {
            let (x, y) = adjust(program, x2_, y2_, false, true);
            if (x, y) != (x2, y2) { x2 = x; y2 = y; changed = true; }
        } else {
            let (x, y) = adjust(program, x2_, y2_, true, false);
            if (x, y) != (x2, y2) { x2 = x; y2 = y; changed = true; }
        }
    }

    //  here both points are affected, just inside the boundary, and difference is exactly n
    //  still this may not be the solution:
    //
    //  ....
    //     ....oo.
    //        .oo......
    //           ...........
    //println!("{:?}", ((x1, y1), (x2, y2)));

    //  try to move both left and up
    let mut x2_ = x2;
    let mut y2_ = y2;
    let mut changed = true;
    while changed {
        changed = false;
        //  move lower-left corner left while possible
        while is_affected(program, x2 - 1, y2) {
            x2 -= 1;
        }
        //  check upper-right corner and save it it's good
        if is_affected(program, x2 + n, y2 - n) {
            changed = true;
            x2_ = x2;
            y2_ = y2;
            //println!("saved {:?}", (x2_, y2_));
        }
        //  move up 1 step
        y2 -= 1;
    }

    ((x2_ + n, y2_ - n), (x2_, y2_))
}

