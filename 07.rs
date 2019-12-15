mod intcode;


fn main() {
    let program = intcode::read("07.input");

    //  part 1
    let r1 = max_thrust(&program, 5);
    println!("{}", r1);

    //  part 2
}


fn amplify(program: &intcode::Program, phase: i32, input: i32) -> i32 {
    let program = program.to_vec();   //  create copy
    intcode::run(program, vec![phase, input])[0]
}

fn amplify_chain(program: &intcode::Program, phases: &Vec<i32>, mut input: i32) -> i32 {
    for phase in phases {
        input = amplify(program, *phase, input);
    }
    input
}

fn max_thrust(program: &intcode::Program, n: i32) -> i32 {
    let mut used = vec![false; n as usize];
    let mut phases = Vec::new();
    let mut results = Vec::new();
    max_thrust_(program, &mut used, &mut phases, &mut results);
    *results.iter().max().unwrap()
}

fn max_thrust_(
    program: &intcode::Program, used: &mut Vec<bool>,
    phases: &mut Vec<i32>, results: &mut Vec<i32>)
{
    if phases.len() == used.len() {
        let result = amplify_chain(program, phases, 0);
        println!("{:?}: {:?}", phases, result);
        results.push(result);
    } else {
        for i in 0 .. used.len() {
            if !used[i] {
                used[i] = true;
                phases.push(i as i32);
                max_thrust_(program, used, phases, results);
                phases.pop();
                used[i] = false;
            }
        }
    }
}
