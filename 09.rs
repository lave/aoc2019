mod intcode;


fn main() {
    let program = intcode::read("09.input");

    //  part 1
    let r1 = intcode::run(&program, vec![1]);
    println!("{}", r1[0]);

    //  part 2
    let r2 = intcode::run(&program, vec![2]);
    println!("{}", r2[0]);
}

