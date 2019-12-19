mod intcode;


fn main() {
    let mut program = intcode::read("02.input");

    //  part 1
    let r1 = process(&mut program, 12, 2);
    println!("{}", r1);

    //  part 2
    let (noun, verb) = find(&mut program, 19690720).unwrap();
    println!("{}", noun * 100 + verb);
}


fn process(program: &mut intcode::Program, noun: i32, verb: i32) -> i32 {
    program[1] = noun;
    program[2] = verb;
    intcode::run0(program, 0)
}


fn find(program: &mut intcode::Program, target: i32) -> Option<(i32, i32)> {
    for noun in 0 .. 99 {
        for verb in 0 .. 99 {
            if process(program, noun, verb) == target {
                return Some((noun, verb));
            }
        }
    }
    None
}
