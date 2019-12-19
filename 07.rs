use std::collections::VecDeque;

mod intcode;


fn main() {
    let program = intcode::read("07.input");

    //  part 1
    let r1 = max_thrust(&program);
    println!("{}", r1);

    //  part 2
    let r2 = max_thrust_fb(&program);
    println!("{}", r2);
}


fn max_thrust(program: &intcode::Program) -> i32 {
    permutations((0 ..= 4).collect()).iter()
        .map(|p| amplify_chain(program, p))
        .max().unwrap()
}

fn amplify_chain(program: &intcode::Program, phases: &Vec<i32>) -> i32 {
    let mut input = 0;
    for phase in phases {
        input = intcode::run(program, vec![*phase, input])[0]
    }
    input
}


fn max_thrust_fb(program: &intcode::Program) -> i32 {
    permutations((5 ..= 9).collect()).iter()
        .map(|p| amplify_chain_fb(program, p))
        .max().unwrap()
}

fn amplify_chain_fb(program: &intcode::Program, phases: &Vec<i32>) -> i32 {
    let n = phases.len();
    let mut result = None;
    let mut programs: VecDeque<intcode::State> = phases.iter()
        .map(| phase | intcode::run_async(intcode::init(program), vec![*phase]).1)
        .collect();
    let mut input = 0;
    let mut i = 0;
    loop {
        i += 1;
        let state = programs.pop_front().unwrap();
        let (output, state_) = intcode::run_async(state, vec![input]);
        if output.is_empty() {
            break;
        } else {
            programs.push_back(state_);
            input = output[0];
        }
        if i % n == 0 {
            result = Some(input);
        }
    }
    result.unwrap()
}


fn permutations(values: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    permutations_(&values, &mut Vec::new(), &mut vec![false; values.len()], &mut result);
    result
}

fn permutations_(values: &Vec<i32>, current: &mut Vec<i32>, used: &mut Vec<bool>, results: &mut Vec<Vec<i32>>) {
    if current.len() == used.len() {
        results.push(current.clone());
    } else {
        for i in 0 .. used.len() {
            if !used[i] {
                used[i] = true;
                current.push(values[i]);
                permutations_(values, current, used, results);
                current.pop();
                used[i] = false;
            }
        }
    }
}

