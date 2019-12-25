use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

pub type Word = i64;
pub type Program = Vec<Word>;
pub type Data = Vec<Word>;
pub type State = (Program, Word, Word);
pub type Result = (Data, State);


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
        .map(|s| s.parse::<Word>().unwrap())
        .collect()
}

#[allow(dead_code)]
pub fn run0(program: &Program, res: Word) -> Word {
    let (_, state) = run_(init(program), Vec::new());
    state.0[res as usize]
}

#[allow(dead_code)]
pub fn run(program: &Program, input: Data) -> Data {
    let (output, _) = run_(init(program), input);
    output
}

#[allow(dead_code)]
pub fn init(program: &Program) -> State {
    (program.clone(), 0, 0)
}

#[allow(dead_code)]
pub fn run_async(state: State, input: Data) -> Result {
    run_(state, input)
}


fn run_((mut program, mut ip, mut rel_base): State, input: Data) -> Result {
    let mut output = Vec::new();
    let mut i = 0;
    loop {
        let opcode = program[ip as usize];
        let modes = opcode / 100;
        let opcode = opcode % 100;

        let mut ctx = (&mut program, ip, rel_base, modes);

        ip = match opcode {
            1 => {  //  add
                let a1 = load(&ctx, 1);
                let a2 = load(&ctx, 2);
                save(&mut ctx, 3, a1 + a2);
                ip + 4
            },
            2 => {  //  mul
                let a1 = load(&ctx, 1);
                let a2 = load(&ctx, 2);
                save(&mut ctx, 3, a1 * a2);
                ip + 4
            },
            3 => {  //  input
                if i < input.len() {
                    //println!("read input: {}", input[i]);
                    save(&mut ctx, 1, input[i]);
                    i += 1;
                    ip + 2
                } else {
                    break;
                }
            },
            4 => {  //  output
                let a = load(&ctx, 1);
                output.push(a);
                ip + 2
            },
            5 => {  //  jump-if-true
                let a1 = load(&ctx, 1);
                if a1 != 0 {
                    load(&ctx, 2)
                } else {
                    ip + 3
                }
            },
            6 => {  //  jump-if-false
                let a1 = load(&ctx, 1);
                if a1 == 0 {
                    load(&ctx, 2)
                } else {
                    ip + 3
                }
            },
            7 => {  //  less
                let a1 = load(&ctx, 1);
                let a2 = load(&ctx, 2);
                save(&mut ctx, 3, if a1 < a2 { 1 } else { 0 });
                ip + 4
            },
            8 => {  //  equals
                let a1 = load(&ctx, 1);
                let a2 = load(&ctx, 2);
                save(&mut ctx, 3, if a1 == a2 { 1 } else { 0 });
                ip + 4
            },
            9 => {  //  adjust relative base
                let a1 = load(&ctx, 1);
                rel_base += a1;
                ip + 2
            },
            99 => break,
            _ => panic!("unknown opcode {}", opcode)
        };
    }

    (output, (program, ip, rel_base))
}


#[allow(dead_code)]
pub fn has_terminated((program, ip, _): &State) -> bool {
    return program[*ip as usize] == 99;
}


fn load((program, ip, rel_base, modes): &(&mut Program, Word, Word, Word), n: u8) -> Word {
    let v = program[(*ip + n as Word) as usize];
    let p = 10_u32.pow(n as u32 - 1);
    let mode = (*modes as u32 / p) % 10;
    match mode {
        0 => get(program, v as usize),
        1 => v,
        2 => get(program, (*rel_base + v) as usize),
        _ => panic!("unknown address mode {}", mode)
    }
}

fn get(program: &Program, addr: usize) -> Word {
    if program.len() <= addr { 0 } else { program[addr] }
}

fn save((program, ip, rel_base, modes): &mut(&mut Program, Word, Word, Word), n: u8, v_: Word) {
    let v = program[(*ip + n as Word) as usize];
    let p = 10_u32.pow(n as u32 - 1);
    let mode = (*modes as u32 / p) % 10;
    let addr = match mode {
        0 => v as usize,
        1 => panic!("mode 1 is not supported for write"),
        2 => (*rel_base + v) as usize,
        _ => panic!("unknown address mode {}", mode)
    };
    if program.len() <= addr {
        program.resize(addr + 1, 0);
    }
    program[addr] = v_;
}


#[allow(dead_code)]
pub fn to_ascii(data: &[Word]) -> String {
    data.iter().map(|&v| v as u8 as char).collect::<String>()
}

#[allow(dead_code)]
pub fn to_asciis(data: &[Word]) -> Vec<String> {
    to_ascii(data).split("\n").map(|s| s.to_string()).collect()
}

#[allow(dead_code)]
pub fn from_ascii(string: &str) -> Data {
    string.chars().map(|v| v as u8 as Word).collect::<Data>()
}

#[allow(dead_code)]
pub fn from_asciis(strings: &Vec<&str>) -> Data {
    from_ascii(&(strings.join("\n") + "\n"))
}

