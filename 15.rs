use std::io;

#[allow(unused_imports)]
use std::time;
#[allow(unused_imports)]
use std::thread;

mod intcode;

type Map = (Vec<i8>, i32);
type Coord = (i32, i32);
type System = (Map, Coord, intcode::State);



fn main() {
    let program = intcode::read("15.input");
    let n = 100;

    //  part 1
    //let system = run_manual(init(&program, n));
    let (system, ox, l) = run_auto(init(&program, n));
    println!("{}", l);

    //  part 2
    let (mut map, _, _) = system;
    let t = fill_ox(&mut map, ox); 
    println!("{}", t);
}


fn init(program: &intcode::Program, n: i32) -> System {
    let n_ = (n * 2 + 1) as usize;
    let mut map = vec![-1; n_ * n_];
    let c = (0, 0);
    map[idx(n, &c)] = 1;
    ((map, n), c, intcode::init(program))
}

fn idx(n: i32, (x, y): &Coord) -> usize {
     ((y + n) * (n * 2 + 1) + (x + n)) as usize
}

fn opposite(mov: i8) -> i8 {
    match mov {
        1 => 2,
        2 => 1,
        3 => 4,
        4 => 3,
        _ => panic!("bad move")
    }
}

fn move_coord((x, y): &Coord, mov: i8) -> Coord {
    match mov {
        1 => (*x, *y - 1),
        2 => (*x, *y + 1),
        3 => (*x - 1, *y),
        4 => (*x + 1, *y),
        _ => panic!("bad move")
    }
}

fn move_(((mut map, n), c, state): System, mov: i8) -> (System, bool, bool) {
    let input = vec![mov as intcode::Word];
    let (output, state_) = intcode::run_async(state, input);

    assert!(!intcode::has_terminated(&state_), "program terminated");
    assert!(output.len() == 1, "invalid output length");
    let o = output[0] as i8;

    let c_ = move_coord(&c, mov);
    let i = idx(n, &c_);
    assert!(map[i] == -1 || map[i] == o, "inconsistent cell type");
    map[i] = o;

    let moved = o > 0;
    let found = o == 2;
    let system_ = ((map, n), if moved { c_ } else { c }, state_);
    (system_, moved, found)
}


#[allow(dead_code)]
fn print(((map, n), c, _): &System) {
    print_(map, *n, c);
}

fn print_(map: &Vec<i8>, n: i32, (x, y): &Coord) {
    let mut y_ = -n;
    for line in map.chunks((n * 2 + 1) as usize) {
        if (y_ - y).abs() <= 1
            || line.iter().any(|v| *v > -1) {
            let mut x_ = -n - 1;
            println!("{}", line.iter().map(|v| {
                x_ += 1;
                if x_ == *x && y_ == *y {
                    if *v == 2 { 'X' } else { 'D' }
                } else {
                    match *v {
                        -1 => ' ',
                        0 => '#',
                        1 => '.',
                        2 => 'X',
                        3 => 'O',
                        _ => panic!("unknown cell type")
                    }
                }
            }).collect::<String>());
        }
        y_ += 1;
    }
}


#[allow(dead_code)]
fn run_manual(mut system: System) -> System {
    loop {
        print(&system);

        let mov = user_move();

        let (system_, _, found) = move_(system, mov);
        system = system_;

        if found { break; }
    }
    system
}

#[allow(dead_code)]
fn user_move() -> i8 {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    match input_line.trim().as_ref() {
        "w" => 1,
        "s" => 2,
        "a" => 3,
        "d" => 4,
        _ => panic!("invalid input: {}", input_line)
    }
}


#[allow(dead_code)]
fn run_auto(mut system: System) -> (System, Coord, u32) {
    let mut ox = None;
    let mut l = None;
    let mut moves = Vec::new();
    loop {
        //  uncomment to see game screen
        //print(&system);
        //  incomment to control speed
        //thread::sleep(time::Duration::from_millis(1));

        let mov = (1 ..= 4).find(|m| is_unknown(&system, *m));
        let mov = match mov {
            None => if moves.is_empty() { None } else { Some(opposite(moves.pop().unwrap())) },
            Some(m) => { moves.push(m); Some(m) }
        };

        if mov.is_none() { break; }

        let (system_, moved, found) = move_(system, mov.unwrap());
        system = system_;

        if !moved { moves.pop(); }

        //println!("{} {}, stack size {}", if moved { "moved" } else { "can't move"}, mov.unwrap(), moves.len());

        if found {
            ox = Some(system.1.clone());
            l = Some(moves.len() as u32);
        }
    }
    (system, ox.unwrap(), l.unwrap())
}

fn is_unknown(((map, n), c, _): &System, mov: i8) -> bool {
    let c_ = move_coord(c, mov);
    map[idx(*n, &c_)] == -1
}


fn fill_ox((map, n): &mut Map, c: Coord) -> u32 {
    let mut t = 0;
    let mut cs = vec![c];
    while !cs.is_empty() {
        let mut cs_ = Vec::new();
        for c in cs {
            map[idx(*n, &c)] = 3;
            let mut new_ = (1 ..= 4)
                .map(|m| move_coord(&c, m))
                .filter(|c_| map[idx(*n, c_)] == 1)
                .collect::<Vec<_>>();
            cs_.append(&mut new_);
        }
        cs = cs_;
        t += 1;

        //print_(map, *n, &(0, 0));
        //  incomment to control speed
        //thread::sleep(time::Duration::from_millis(10));
    }
    t - 1
}
