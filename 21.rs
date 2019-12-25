mod intcode;


fn main() {
    let program = intcode::read("21.input");

    //  part 1
    let script = vec![
        //  test that we need to jump (there's a jole in next 3 tiles), and we can jump (we land in
        //  a ground tile):
        //      * ??#
        //      *? ?#
        //      *?? #
        "NOT J J",  //  J = TRUE
        "AND A J",
        "AND B J",
        "AND C J",  //  J = FALSE iff need to jump
        "NOT J J",  //  J = TRUE  iff need to jump
        "AND D J"   //  J = TRUE  iff need to jump and it's safe
    ];
    let damage = run_droid(&program, &script, false);
    println!("{}", damage);

    //  part 2
    let script = vec![
        //  test that we need to jump (there's a jole in next 3 tiles), and we can jump (we land in
        //  a ground tile):
        //      * ??#
        //      *? ?#
        //      *?? #
        "NOT J J",  //  J = TRUE
        "AND A J",
        "AND B J",
        "AND C J",  //  J = FALSE iff need to jump
        "NOT J J",  //  J = TRUE  iff need to jump
        "AND D J",  //  J = TRUE  iff need to jump and it's safe
        //  additionally test that we can make 2nd jump - we can tell 3 cases with available
        //  sensors:
        //      jump from D: *???#???#
        //      jump from E: *???##???#
        //      jump from F: *???###???
        //  they can be encoded as:
        //      H | E & I | E & F
        //  or, equivalently:
        //      H | E & (I | F)
        "OR  I T",
        "OR  F T",
        "AND E T", 
        "OR  H T",  //  T = TRUE  iff it's possible to make 2nd jump from D, E or F
        "AND T J"   //  J = TRUE  iff we need to jump, it's safe, and we can make 2nd jump after that
    ];
    let damage = run_droid(&program, &script, true);
    println!("{}", damage);
}


fn run_droid(program: &intcode::Program, script: &Vec<&str>, is_run: bool) -> u32 {
    let (output, state) = intcode::run_async(intcode::init(program), intcode::Data::new());
    let output_str = intcode::to_ascii(&output);
    assert!(output_str == "Input instructions:\n");

    let mut script = script.clone();
    script.push(if is_run { "RUN" } else { "WALK" });
    let input = intcode::from_asciis(&script);
    let (output, _state) = intcode::run_async(state, input);

    let common = &output[0 .. 13];
    assert!(intcode::to_ascii(common)
        == if is_run { "\nRunning...\n\n" } else { "\nWalking...\n\n" });

    let damage;
    if output.len() == 14 {
        damage = Some(output[13] as u32);
    } else {
        let output_str = intcode::to_ascii(&output[13 ..]);
        println!("{}", output_str);
        panic!("springdroid didn't make it through");
    }

    damage.unwrap()
}
