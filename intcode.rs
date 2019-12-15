use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use either::Either;

pub type Program = Vec<i32>;
pub type Data = Vec<i32>;
pub type Result = Either<Data, Box<Fn<Data, Data>>>;


#[allow(dead_code)]
pub fn read(filename: &str) -> Program {
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);
    let line = file.lines().next().unwrap().unwrap();
    parse(&line)
}

#[allow(dead_code)]
pub fn parse(program: &str) -> Program {
    program
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

#[allow(dead_code)]
pub fn run0(program: Program, res: i32) -> i32 {
    run_(program, Vec::new(), Some(res))[0]
}

#[allow(dead_code)]
pub fn run(program: Program, input: Data) -> Data {
    run_(program, input, None)
}

pub fn run_async(program: Program, input: Data) ->

fn run_(mut program: Program, input: Data, res: Option<i32>) -> Data {
    let mut output = Vec::new();
    let mut i = 0;
    let mut j = 0;
    loop {
        let opcode = program[i];
        let modes = opcode / 100;
        let opcode = opcode % 100;

        let mut ctx = (&mut program, i, modes);

        let di = match opcode {
            1 => {  //  add
                let a1 = load(&ctx, 1);
                let a2 = load(&ctx, 2);
                save(&mut ctx, 3, a1 + a2);
                4 },
            2 => {  //  mul
                let a1 = load(&ctx, 1);
                let a2 = load(&ctx, 2);
                save(&mut ctx, 3, a1 * a2);
                4 },
            3 => {  //  input
                save(&mut ctx, 1, input[j]);
                j += 1;
                2 },
            4 => {  //  output
                let a = load(&ctx, 1);
                output.push(a);
                2 },
            5 => {  //  jump-if-true
                let a1 = load(&ctx, 1);
                if a1 != 0 {
                    i = load(&ctx, 2) as usize; 0
                } else { 3 }
                },
            6 => {  //  jump-if-false
                let a1 = load(&ctx, 1);
                if a1 == 0 {
                    i = load(&ctx, 2) as usize; 0
                } else { 3 }
                },
            7 => {  //  less
                let a1 = load(&ctx, 1);
                let a2 = load(&ctx, 2);
                save(&mut ctx, 3, if a1 < a2 { 1 } else { 0 });
                4 },
            8 => {  //  equals
                let a1 = load(&ctx, 1);
                let a2 = load(&ctx, 2);
                save(&mut ctx, 3, if a1 == a2 { 1 } else { 0 });
                4 },
            99 => break,
            _ => panic!("unknown opcode {}", opcode)
        };
        i += di;
    }

    if res.is_some() {
        output.push(program[res.unwrap() as usize]);
    }
    output
}

fn load((program, i, modes): &(&mut Program, usize, i32), n: u8) -> i32 {
    let v = program[*i + n as usize];
    let p = 10_u32.pow(n as u32 - 1);
    let mode = (*modes as u32 / p) % 10;
    match mode {
        0 => program[v as usize],
        1 => v,
        _ => panic!("unknown mode {}", mode)
    }
}

fn save((program, i, _modes): &mut(&mut Program, usize, i32), n: u8, v: i32) {
    let i = program[*i + n as usize];
    program[i as usize] = v;
}
