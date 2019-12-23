use std::io;

#[allow(unused_imports)]
use std::time;
#[allow(unused_imports)]
use std::thread;

mod intcode;

type Map = Vec<i8>;
type Dims = (i64, i64, i64, i64);



fn main() {
    let mut program = intcode::read("13.input");

    //  part 1
    let (map, dims) = run(&program);
    let blocks_count = map.iter().filter(|v| **v == 2).count();
    println!("{}", blocks_count);

    //  part 2
    program[0] = 2;
    let score = run_interactive(&program, dims);
    println!("{}", score);
}


fn run(program: &intcode::Program) -> (Map, Dims) {
    let output = intcode::run(program, Vec::new());

    let n = output.len();
    assert!(n > 0, "empty output");
    assert!(n % 3 == 0, "invalid output");
    let n = n / 3 ;

    let mut minx = output[0];
    let mut maxx = output[0];
    let mut miny = output[1];
    let mut maxy = output[1];

    for i in 0 .. n {
        let x = output[i * 3];
        let y = output[i * 3 + 1];

        if x < minx { minx = x };
        if x > maxx { maxx = x };
        if y < miny { miny = y };
        if y > maxy { maxy = y };
    }
    //println!("{} .. {} X {} .. {}", minx, maxx, miny, maxy);

    let w = maxx - minx + 1;
    let h = maxy - miny + 1;
    let mut map = vec![0; (w * h) as usize];
    let mut score = 0;

    draw(&mut map, &mut score, &output, w, minx, miny);

    (map, (minx, maxx, miny, maxy))
}


fn run_interactive(program: &intcode::Program, dims: Dims) -> i64 {
    let (minx, maxx, miny, maxy) = dims;

    let w = maxx - minx + 1;
    let h = maxy - miny + 1;
    let mut map = vec![0; (w * h) as usize];
    let mut score = 0;

    let mut state = intcode::init(program);
    let mut input = Vec::new();
    loop {
        let (output, state_) = intcode::run_async(state, input);
        state = state_;

        draw(&mut map, &mut score, &output, w, minx, miny);

        //  uncomment to see game screen
        //print_map(&map, w as usize, h as usize);

        if intcode::has_terminated(&state) { break; }

        //  uncomment one of these to switch between interactive/auto modes
        //let mov = user_move();
        let mov = auto_move(&map, w);

        //  incomment to control speed
        //thread::sleep(time::Duration::from_millis(10));

        input = vec![mov as intcode::Word];
    }
    score
}


fn draw(
    map: &mut Map, score: &mut i64, output: &intcode::Data,
    w: i64, minx: i64, miny: i64)
{
    let n = output.len();
    assert!(n % 3 == 0, "invalid output");
    let n = n / 3 ;

    for i in 0 .. n {
        let x = output[i * 3];
        let y = output[i * 3 + 1];
        let v = output[i * 3 + 2];

        if x == -1 && y == 0 {
            *score = v;
        } else {
            let c = (y - miny) * w + (x - minx);
            map[c as usize] = v as i8;
        }
    }
}


#[allow(dead_code)]
fn print_map(map: &Map, w: usize, _h: usize) {
    for line in map.chunks(w) {
        println!("{}", line.iter().map(
            |v| match v {
                0 => ' ',
                1 => '#',
                2 => '.',
                3 => '-',
                4 => 'o',
                _ => panic!("unknown tile")
            }
        ).collect::<String>());
    }
}

#[allow(dead_code)]
fn user_move() -> i8 {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    match input_line.trim().as_ref() {
        "a" => -1,
        "" => 0,
        "d" => 1,
        _ => panic!("invalid input: {}", input_line)
    }
}

#[allow(dead_code)]
fn auto_move(map: &Map, w: i64) -> i8 {
    let mut ball_pos = -1;
    let mut paddle_pos = -1;
    for i in 0 .. map.len() {
        if map[i] == 3 { paddle_pos = i as i64; }
        if map[i] == 4 { ball_pos = i as i64; }
    }
    let ball_x = ball_pos % w;
    let paddle_x = paddle_pos % w;

    if ball_x < paddle_x { -1 }
    else if ball_x > paddle_x { 1 }
    else { 0 }
}
