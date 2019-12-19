mod intcode;


fn main() {
    let program = intcode::read("05.input");

    //  part 1
    let r1 = run_test(&program, 1);
    println!("{}", r1);

    //  part 2
    let r2 = run_test(&program, 5);
    println!("{}", r2);
}


fn run_test(program: &intcode::Program, id: i32) -> i32 {
    let output = intcode::run(program, vec![id]);
    *output.last().unwrap()
}

