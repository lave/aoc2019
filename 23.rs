mod intcode;


type Nic = (intcode::State, intcode::Data);


fn main() {
    let program = intcode::read("23.input");

    //  part 1
    let nics = create(&program, 50);
    let (first_y, last_y) = run(nics);
    println!("{}", first_y);

    //  part 2
    println!("{}", last_y);
}


fn create(program: &intcode::Program, n: u32) -> Vec<Nic> {
    let mut nics = Vec::new();
    for i in 0 .. n {
        nics.push((intcode::init(program), vec![i as intcode::Word]));
    }
    nics
}


fn run(mut nics: Vec<Nic>) -> (intcode::Word, intcode::Word) {
    let mut first_y = None;
    let mut last_y = None;

    let mut nat = None;
    loop {
        let mut states = Vec::new();
        let mut inputs = vec![intcode::Data::new(); nics.len()];
        let mut inputs_empty = true;
        let mut outputs_empty = true;
        for (state, input) in nics {
            if !(input.len() == 1 && input[0] == -1) { inputs_empty = false; }
            let (output, state_) = intcode::run_async(state, input);
            states.push(state_);
            
            assert!(output.len() % 3 == 0);
            for i in 0 .. output.len() / 3 {
                outputs_empty = false;
                let addr = output[i * 3];
                let x = output[i * 3 + 1];
                let y = output[i * 3 + 2];

                //println!("sent {}:{} to {}", x, y, addr);

                if addr == 255 {
                    if first_y.is_none() { first_y = Some(y); }
                    nat = Some((x, y));
                } else {
                    let input = &mut inputs[addr as usize];
                    input.push(x);
                    input.push(y);
                }
            }
        }

        if inputs_empty && outputs_empty {
            let (x, y) = nat.unwrap();
            inputs[0].push(x);
            inputs[0].push(y);

            //println!("nat sent {}:{} to {}", x, y, 0);

            if last_y.is_some() && y == last_y.unwrap() {
                break;
            } else {
                last_y = Some(y);
            }
        }

        for input in &mut inputs {
            if input.is_empty() {
                input.push(-1);
            }
        }

        nics = states.into_iter().zip(inputs).collect();
    }

    (first_y.unwrap(), last_y.unwrap())
}

