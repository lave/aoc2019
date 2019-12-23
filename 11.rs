mod intcode;

type Map = Vec<i8>;
type Visited = Vec<bool>;



fn main() {
    let program = intcode::read("11.input");

    let n = 100;
    let n_ = n * 2 + 1;

    //  part 1
    let mut map = vec![0; n_ * n_];
    let visited = run_robot(&program, &mut map, n as i32);
    let visited_count = visited.iter().filter(|v| **v).count();
    println!("{}", visited_count);

    //  part 2
    let mut map = vec![0; n_ * n_];
    map[map_coord(n as i32, &(0, 0))] = 1;
    run_robot(&program, &mut map, n as i32);
    print_map(&map, n_);
}


fn run_robot(program: &intcode::Program, map: &mut Map, n: i32) -> Visited {
    let mut robot = ((0, 0), 0);
    let mut visited = vec![false; map.len()];

    let mut state = intcode::init(&program);
    let mut input = Vec::new();
    loop {
        let (output, state_) = intcode::run_async(state, input);
        assert!(output.len() % 2 == 0);
        state = state_;

        let (coord, dir) = &mut robot;
        for i in 0 .. output.len() / 2 {
            //  paint
            let coord_ = map_coord(n, coord);
            map[coord_] = output[i * 2] as i8;
            visited[coord_] = true;
            //println!("paint {:?} to {:?}", coord, output[i * 2]);

            //  turn (map 0 to -1 and 1 to 1)
            let turn = (output[i * 2 + 1] as i8) * 2 - 1;
            *dir = (*dir + turn + 4) % 4;
            //println!("turn {:?}: new dir is {:?}", output[i * 2 + 1], dir);

            //  move - 0 = up, 1 = right, 2 = down, 3 = left
            match dir {
                0 => coord.1 -= 1,
                1 => coord.0 += 1,
                2 => coord.1 += 1,
                3 => coord.0 -= 1,
                _ => panic!("invalid direction {}", dir)
            };
            //println!("move to {:?}", coord);
        }

        if intcode::has_terminated(&state) { break; }

        input = vec![map[map_coord(n, coord)] as intcode::Word];
    }
    visited
}

fn map_coord(n: i32, (x, y): &(i32, i32)) -> usize {
    assert!(*x >= -n && *x <= n && *y >= -n && *y <= n);
    ((y + n) * (n * 2 + 1) + (x + n)) as usize
}


fn print_map(map: &Map, n: usize) {
    for line in map.chunks(n) {
        if line.iter().any(|v| *v == 1) {
            println!("{}", line.iter().map(
                |v| if *v == 0 { ' ' } else { '#' }
            ).collect::<String>());
        }
    }
}
