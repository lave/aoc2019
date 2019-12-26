use std::io;

mod intcode;


fn main() {
    let program = intcode::read("25.input");

    //  part 1
    let script = vec![
        "south",
        "take astronaut ice cream",
        "north",
        "east",
        "take mouse",
        "north",
        "take spool of cat6",
        "north",
        "take hypercube",
        "east",
        "take sand",
        "south",
        "take antenna",
        "north",
        "west",
        "south",
        "south",
        "south",
        "take mutex",
        "west",
        "take boulder",
        "south",
        "south",
        "south",
        "west",
        "south",
        "drop astronaut ice cream",
        "drop mouse",
        "drop spool of cat6",
        "drop hypercube",
        "drop sand",
        "drop antenna",
        "drop mutex",
        "drop boulder"
    ];

    let password = run(&program, script);
    println!("{}", password);
}


fn run(program: &intcode::Program, script: Vec<&str>) -> String {
    let mut state = intcode::init(program);
    let mut input = intcode::Data::new();
    let mut script_idx = 0;
    loop {
        let (output, state_) = intcode::run_async(state, input);
        state = state_;
        println!("{}", intcode::to_ascii(&output));

        let mut input_line = String::new();
        if script_idx < script.len() {
            input_line = script[script_idx].to_string() + "\n";
            script_idx += 1;
        } else {
            io::stdin().read_line(&mut input_line).unwrap();
        }

        if input_line == "exit\n" { break; }

        if input_line == "try\n" {
            state = try_weights(state, vec![
                "take astronaut ice cream", "take mouse", "take spool of cat6",
                "take hypercube", "take sand", "take antenna", "take mutex", "take boulder"]);
            input_line = "inv\n".to_string();
        }

        input = intcode::from_ascii(&input_line);
    }

    "".to_string()
}


fn try_weights(state0: intcode::State, items: Vec<&str>) -> intcode::State {
    let l = items.len();
    let n = 1 << l;
    let mut state = state0.clone();
    for i in 0 .. n {
        let mut script = Vec::new();
        for j in 0 .. l {
            if (i & (1 << j)) != 0 { script.push(items[j]); }
        }

        let (output, state_) = intcode::run_async(state, intcode::from_asciis(&script));
        state = state_;
        //println!("{}", intcode::to_ascii(&output));

        let (output, state_) = intcode::run_async(state, intcode::from_ascii("south\n"));
        state = state_;
        let output_str = intcode::to_ascii(&output);
        println!("{}", output_str);
        if !output_str.contains("and you are ejected back to the checkpoint") {
            break;
        } else {
            state = state0.clone();
        }
    }
    state
}
