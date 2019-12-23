use std::collections::HashSet;

mod intcode;

type Map = (Vec<i8>, i32);
type Bot = (i32, i32, i8);
type Path = Vec<i8>;


fn main() {
    let mut program = intcode::read("17.input");
    let (map, bot) = build_map(&program);
    //print_map(&map);

    //  part 1
    let alignment = get_alignment(&map);
    println!("{}", alignment.iter().sum::<u32>());

    //  part 2
    let path = build_path(&map, bot);
    //println!("{:?}", path);
    let all_chunks = find_chunks(&path);
    //println!("{:?}", all_chunks);
    let chunks = find_3_chunks(&path, all_chunks);
    //println!("{:?}", chunks);

    let out = run_robot(&mut program, chunks);
    println!("{}", out);
}


fn build_map(program: &intcode::Program) -> (Map, Bot) {
    let output = intcode::run(program, Vec::new());

    let mut w = None;
    let mut bot = None;

    let mut x = 0;
    let mut y = 0;
    let map = output.iter().filter_map(|v| {
        if *v == 10 {
            if w.is_none() {
                w = Some(x);
            } else if x > 0 {
                assert!(x == w.unwrap(), "inconsistent row length");
            }
            y += 1;
            x = 0;
            None
        } else {
            let r = match *v as u8 as char {
                '.' => Some(0),
                '#' => Some(1),
                'X' => { bot = Some((x, y, 0)); Some(0) },
                '^' => { bot = Some((x, y, 1)); Some(1) },
                '>' => { bot = Some((x, y, 2)); Some(1) },
                'v' => { bot = Some((x, y, 3)); Some(1) },
                '<' => { bot = Some((x, y, 4)); Some(1) },
                _ => panic!("unknown char {}", v)
            };
            x += 1;
            r
        }
    }).collect();

    ((map, w.unwrap()), bot.unwrap())
}

#[allow(dead_code)]
fn print_map((map, w): &Map) {
    for line in map.chunks(*w as usize) {
        println!("{}", line.iter().map(|v| {
            match *v {
                0 => '.',
                1 => '#',
                _ => panic!("unknown cell type")
            }
        }).collect::<String>());
    }
}


fn get_alignment((map, w_): &Map) -> Vec<u32> {
    let w = *w_ as usize;
    let mut r = Vec::new();
    let h = map.len() / w;
    for i in 0 .. map.len() {
        let x = i % w;
        let y = i / w;

        if x > 0 && x < w - 1 && y > 0 && y < h - 1
            && map[i] == 1
            && map[i + 1] == 1 && map[i - 1] == 1
            && map[i + w] == 1 && map[i - w] == 1
        {
            r.push((x * y) as u32);
        }
    }
    r
}


//  move bot one step forward if possible
fn fwd((map, w_): &Map, (x, y, d): &Bot) -> Option<Bot> {
    let w = *w_ as i32;
    let h = (map.len() as i32) / w;

    let (x_, y_) = match d {
        1 => (*x, y - 1),
        2 => (x + 1, *y),
        3 => (*x, y + 1),
        4 => (x - 1, *y),
        _ => panic!("unknown dir {}", d)
    };

    if x_ < 0 || x_ >= w || y_ < 0 || y_ >= h || map[(y_ * w + x_) as usize] == 0 {
        None
    } else {
        Some((x_, y_, *d))
    }
}

fn build_path(map: &Map, mut bot: Bot) -> Path {
    let mut path = Vec::new();
    loop {
        let (x, y, d) = bot;
        let d_l = (d - 2 + 4) % 4 + 1;
        let bot_l = fwd(map, &(x, y, d_l));

        let d_r = d % 4 + 1;
        let bot_r = fwd(map, &(x, y, d_r));

        assert!(bot_l.is_none() || bot_r.is_none(), "there's fork");

        //  no way to both sides - end of the path
        if bot_l.is_none() && bot_r.is_none() { break; }

        let is_r = bot_l.is_none();
        let d_ = if is_r { d_r } else { d_l };
        bot = (x, y, d_);
        let mut l = 0;
        loop {
            let bot_ = fwd(map, &bot);
            if bot_.is_none() { break; }
            else { bot = bot_.unwrap(); l += 1; }
        }
        assert!(l > 0);

        path.push(if is_r { l } else { -l });
    }
    path
}


fn find_chunks(path: &Path) -> Vec<Path> {
    let mut chunks = HashSet::new();

    let n = path.len();
    for i in 0 .. n {
        let mut chunk = Vec::new();
        let mut l = 0;
        for j in (i + 1) .. n {
            let v = path[j];
            l +=  (l > 0) as u8     //  for comma
                + 2                 //  for L,
                + (if v.abs() < 10 { 1 } else { 2 });
            if l > 20 { break; }
            chunk.push(v);
            chunks.insert(chunk.clone());
        }
    }
    chunks.drain().collect()
}


fn find_3_chunks(path: &Path, chunks: Vec<Path>) -> Vec<Path> {
    let n = chunks.len();
    let mut r = None;
    'outer: for i in 0 .. n {
        for j in i + 1 .. n {
            for k in j + 1 .. n {
                //println!("trying chunks {}, {}, {}", i, j, k);
                match chunks_good(path, &chunks[i], &chunks[j], &chunks[k]) {
                    Some(path) => {
                        r = Some(vec![path, chunks[i].clone(), chunks[j].clone(), chunks[k].clone()]);
                        break 'outer;
                    },
                    None => {}
                }
            }
        }
    }
    r.unwrap()
}

fn chunks_good(path: &Path, a: &Path, b: &Path, c: &Path) -> Option<Path> {
    let n = path.len();
    let na = a.len();
    let nb = b.len();
    let nc = c.len();

    let mut i = 0;
    let mut l = 0;
    let mut s = Vec::new();
    while i < n && l < 20 {
        if path[i .. i + na] == a[..] { s.push(0); i += na; l += 2; }
        else if path[i .. i + nb] == b[..] { s.push(1); i += nb; l += 2; }
        else if path[i .. i + nc] == c[..] { s.push(2); i += nc; l += 2; }
        else { break; }
    }
    if i == n { Some(s) } else { None }
}


fn run_robot(program: &mut intcode::Program, chunks: Vec<Path>) -> i64 {
    program[0] = 2;

    assert!(chunks.len() == 4);

    let mut input = intcode::Data::new();
    main_prg(&chunks[0], &mut input);
    mov_prg(&chunks[1], &mut input);
    mov_prg(&chunks[2], &mut input);
    mov_prg(&chunks[3], &mut input);
    
    input.push('n' as i64);
    input.push(10);

    let output = intcode::run(&program, input);
    *output.last().unwrap()
}

fn main_prg(chunk: &Path, buffer: &mut intcode::Data) {
    let mut need_comma = false;
    for i in 0 .. chunk.len() {
        if need_comma { buffer.push(',' as i64); } else { need_comma = true; }
        buffer.push('A' as i64 + chunk[i] as i64);
    }
    buffer.push(10);
}

fn mov_prg(chunk: &Path, buffer: &mut intcode::Data) {
    let mut need_comma = false;
    for i in 0 .. chunk.len() {
        let v = chunk[i] as i64;
        if need_comma { buffer.push(',' as i64); } else { need_comma = true; }
        buffer.push((if v < 0 { 'L' } else { 'R' }) as i64);
        buffer.push(',' as i64);
        let v = v.abs();
        if v > 10 { buffer.push('0' as i64 + v / 10); }
        buffer.push('0' as i64 + v % 10);
    }
    buffer.push(10);
}
