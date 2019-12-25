use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;

type Coord = (usize, usize);
type BitMap = Vec<Vec<u8>>;
type Map = (BitMap, usize, usize);
type Visited = Vec<Vec<bool>>;
type Label = String;
type Entrances = HashMap<Coord, Coord>;

type Coord3 = (usize, usize, usize);
type Visited3 = Vec<Vec<Vec<bool>>>;


fn main() {
    let (map, entrances, start, finish) = parse("20.input");
    let (x0, y0) = start;
    let (x1, y1) = finish;
    //println!("{:?}", map);
    //println!("{:?}", entrances);
    //println!("{:?} -> {:?}", start, finish);

    //  part 1
    let steps = find_path(&map, &entrances, (x0, y0), (x1, y1));
    println!("{}", steps);

    //  part 2
    let steps = find_path_recursive(&map, &entrances, (x0, y0, 0), (x1, y1, 0));
    println!("{}", steps);
}


fn make_label(c1: char, c2: char) -> Label {
    assert!(c1 >= 'A' && c1 <= 'Z');
    assert!(c2 >= 'A' && c2 <= 'Z');
    //let c1_ = c1 as u8 - 'A' as u8;
    //let c2_ = c2 as u8 - 'A' as u8;
    format!("{}{}", c1, c2)
}

fn parse(filename: &str) -> (Map, Entrances, Coord, Coord) {

    fn try_label((map, w, h): &Map, x: usize, y: usize, d: u8, labels: &mut HashMap<Label, Vec<Coord>>) {
        let p = match d {
            0 => if y > 0     { Some((x, y - 1)) } else { None },
            1 => if x < w - 1 { Some((x + 1, y)) } else { None },
            2 => if y < h - 1 { Some((x, y + 1)) } else { None },
            3 => if x > 0     { Some((x - 1, y)) } else { None },
            _ => panic!("unknown direction")
        };
        if p.is_some() {
            let (x_, y_) = p.unwrap();
            if map[y_][x_] == 0 {
                let c1 = map[y][x] as char;
                let c2 = map[2 * y - y_][2 * x - x_] as char;
                let label = if d == 1 || d == 2 {
                    make_label(c2, c1)
                } else {
                    make_label(c1, c2)
                };
                (*labels.entry(label.clone()).or_insert(Vec::new())).push((x_, y_));
            }
        }
    }

    let f = File::open(filename).unwrap();
    let lines = BufReader::new(&f).lines();

    let mut letters = Vec::new();

    let mut x = 0;
    let mut y = 0;
    let map = lines.map(|line| {
        let r = line.unwrap().chars()
            .map(|c| {
                let r = match c {
                    '#' => 1,
                    '.' => 0,
                    ' ' => 2,
                    'A' ..= 'Z' => {
                        letters.push((x, y));
                        c as u8
                    },
                    _ => panic!("Unknown char {}", c)
                };
                x += 1;
                r
            }).collect::<Vec<u8>>();
        y += 1;
        x = 0;
        r
    }).collect::<Vec<_>>();

    let w = map[0].len();
    let h = map.len();
    let map = (map, w, h);

    //  assign labels to entrances
    let mut labels = HashMap::new();
    for (x, y) in letters {
        try_label(&map, x, y, 0, &mut labels);
        try_label(&map, x, y, 1, &mut labels);
        try_label(&map, x, y, 2, &mut labels);
        try_label(&map, x, y, 3, &mut labels);
    }
    //println!("{:?}", labels);

    let mut start = None;
    let mut finish = None;
    let mut entrances = HashMap::new();
    for (k, v) in labels {
        match k.as_ref() {
            "AA" => {
                assert!(v.len() == 1);
                start = Some(v[0]);
            },
            "ZZ" => {
                assert!(v.len() == 1);
                finish = Some(v[0]);
            },
            _ => {
                assert!(v.len() == 2);
                entrances.insert(v[0], v[1]);
                entrances.insert(v[1], v[0]);
            }
        }
    };

    (map, entrances, start.unwrap(), finish.unwrap())
}


fn find_path((map, w, h): &Map, entrances: &Entrances, start: Coord, finish: Coord) -> u32 {

    fn try_cell(
        (x, y): Coord, (mut x_, mut y_): Coord, map: &BitMap, entrances: &Entrances,
        visited: &mut Visited, cells: &mut Vec<Coord>) 
    {
        if map[y_][x_] >= 'A' as u8 && map[y_][x_] <= 'Z' as u8 {
            let p = entrances.get(&(x, y));
            if p.is_some() {
                let (x__, y__) = p.unwrap();
                x_ = *x__;
                y_ = *y__;
                //println!("wrapped from {} {} to {} {}", x, y, x__, y__);
            }
        }
        if map[y_][x_] == 0 && !visited[y_][x_] {
            visited[y_][x_] = true;
            cells.push((x_, y_));
        }
    }

    let mut frontier = vec![start];
    let mut visited = vec![vec![false; *w]; *h];
    let mut steps = 0;
    let mut steps_ = None;

    'outer: while !frontier.is_empty() {
        let mut frontier_ = Vec::new();
        for cell in frontier {
            if cell == finish {
                steps_ = Some(steps);
                break 'outer;
            }
            let (x, y) = cell;
            try_cell(cell, (x - 1, y), map, entrances, &mut visited, &mut frontier_);
            try_cell(cell, (x + 1, y), map, entrances, &mut visited, &mut frontier_);
            try_cell(cell, (x, y - 1), map, entrances, &mut visited, &mut frontier_);
            try_cell(cell, (x, y + 1), map, entrances, &mut visited, &mut frontier_);
        }

        //println!("new frontier for {} steps: {:?}", steps, frontier_);
        frontier = frontier_;
        steps += 1;
    }

    steps_.unwrap()
}


fn find_path_recursive(map: &Map, entrances: &Entrances, start: Coord3, finish: Coord3) -> u32 {

    fn is_external((x, y): &Coord, w: usize, h: usize) -> bool {
        *x == 2 || *y == 2 || *x == w - 3 || *y == h - 3
    }

    fn try_cell(
        (x, y, l): Coord3, (mut x_, mut y_, mut l_): Coord3, (map, w, h): &Map, entrances: &Entrances,
        visited: &mut Visited3, cells: &mut Vec<Coord3>) 
    {
        if map[y_][x_] >= 'A' as u8 && map[y_][x_] <= 'Z' as u8 {
            let entrance = (x, y);
            let external = is_external(&entrance, *w, *h);
            if !external || l > 0 {
                let p = entrances.get(&(x, y));
                if p.is_some() {
                    let (x__, y__) = p.unwrap();
                    x_ = *x__;
                    y_ = *y__;
                    l_ = if external { l - 1 } else { l + 1 };
                    //println!("wrapped from {} {} {} to {} {} {}", x, y, l, x_, y_, l_);
                }
            }
        }
        if visited.len() <= l_ {
            visited.push(vec![vec![false; *w]; *h]);
        }
        if map[y_][x_] == 0 && !visited[l_][y_][x_] {
            visited[l_][y_][x_] = true;
            cells.push((x_, y_, l_));
        }
    }

    let mut frontier = vec![start];
    let mut visited = Vec::new();
    let mut steps = 0;
    let mut steps_ = None;

    'outer: while !frontier.is_empty() {
        let mut frontier_ = Vec::new();
        for cell in frontier {
            if cell == finish {
                steps_ = Some(steps);
                break 'outer;
            }
            let (x, y, l) = cell;
            try_cell(cell, (x - 1, y, l), map, entrances, &mut visited, &mut frontier_);
            try_cell(cell, (x + 1, y, l), map, entrances, &mut visited, &mut frontier_);
            try_cell(cell, (x, y - 1, l), map, entrances, &mut visited, &mut frontier_);
            try_cell(cell, (x, y + 1, l), map, entrances, &mut visited, &mut frontier_);
        }

        //println!("new frontier for {} steps: {:?}", steps, frontier_);
        frontier = frontier_;
        steps += 1;
    }

    steps_.unwrap()
}
