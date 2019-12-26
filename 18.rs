use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::cmp::max;
use std::collections::HashMap;

type Coord = (usize, usize);
type PathLength = u32;
type Keys = Vec<Coord>;
type Doors = Vec<Coord>;
type BitMap = Vec<Vec<u8>>;
type Map = (BitMap, usize, usize);
type Visited = Vec<Vec<bool>>;
type Found = u32;
type StateAt = HashMap<Found, PathLength>;
type StatesAt = Vec<StateAt>;


fn main() {
    let (map, keys, doors, start) = parse("18.input");
    //println!("{:?}", map);
    //println!("{:?}", keys);
    //println!("{:?}", doors);
    //println!("{:?}", start);

    //  part 1
    let steps = find_all_keys(&map, &keys, &doors, start);
    println!("{}", steps);

    //  part 2
    let steps = find_all_keys_4(&map, &keys, &doors, start);
    println!("{}", steps);
}


fn parse(filename: &str) -> (Map, Keys, Doors, Coord) {
    let f = File::open(filename).unwrap();
    let lines = BufReader::new(&f).lines();

    let mut start = None;
    let mut keys = vec![(0 ,0); 26];
    let mut doors = vec![(0, 0); 26];
    let mut n = 0;

    let mut x = 0;
    let mut y = 0;
    let map = lines.map(|line| {
        let r = line.unwrap().chars()
            .map(|c| {
                let r = match c {
                    '#' => 1,
                    '.' => 0,
                    '@' => { start = Some((x, y)); 0 },
                    'a'..='z' => {
                        let i = c as u8 - 'a' as u8;
                        n = max(n, i + 1);
                        keys[i as usize] = (x, y);
                        0
                    },
                    'A'..='Z' => {
                        let i = c as u8 - 'A' as u8;
                        n = max(n, i + 1);
                        doors[i as usize] = (x, y);
                        1
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

    keys.resize(n as usize, (0, 0));
    doors.resize(n as usize, (0, 0));

    let w = map[0].len();
    let h = map.len();
    ((map, w, h), keys, doors, start.unwrap())
}


fn find_keys((map, w, h): &Map, keys: &Keys, (x, y): &Coord, found: Found) -> Vec<(usize, PathLength)> {

    fn try_cell((x, y): Coord, map: &BitMap, visited: &mut Visited, cells: &mut Vec<Coord>) {
        if map[y][x] == 0 && !visited[y][x] {
            visited[y][x] = true;
            cells.push((x, y));
        };
    }

    let mut visited = vec![vec![false; *w]; *h];
    visited[*y][*x] = true;
    let mut keys_ = Vec::new();

    let mut d = 0;
    let mut cells = vec![(*x, *y)];
    while !cells.is_empty() {
        let mut cells_ = Vec::new();
        for (x, y) in cells {
            let key = keys.iter().position(|c| *c == (x, y));
            if key.is_some() && !has_key(found, key.unwrap()) { keys_.push((key.unwrap(), d)); }

            if x > 0     { try_cell((x - 1, y), map, &mut visited, &mut cells_); }
            if x < w - 1 { try_cell((x + 1, y), map, &mut visited, &mut cells_); }
            if y > 0     { try_cell((x, y - 1), map, &mut visited, &mut cells_); }
            if y < h - 1 { try_cell((x, y + 1), map, &mut visited, &mut cells_); }
        }
        //println!("new frontier: {:?}", cells_);
        cells = cells_;
        d += 1;
    }
    keys_
}

fn update_state_at(_nkeys: usize, _key: usize, state_at: &mut StateAt, found: Found, steps: PathLength) -> bool {
    //  whether f1 is a superset of f2
    fn is_superset(f1: Found, f2: Found) -> bool {
        f2 & !f1 == 0
    }

    let has_better = state_at.iter()
        .any(|(found_, steps_)| is_superset(*found_, found) && *steps_ <= steps);
    if !has_better {
        state_at.retain(|found_, steps_| !(is_superset(found, *found_) && steps <= *steps_));
        assert!(!state_at.contains_key(&found));
        state_at.insert(found.clone(), steps);
        //println!("Added state for key {:?}: {:?} => {:?}", _key, found_keys_str(found, _nkeys), steps);
        //println!("All states for key {:?}: {:?}", _key, state_at);
    }
    !has_better
}

fn set_door((map, _, _): &mut Map, (x, y): &Coord, is_set: bool) {
    map[*y][*x] = if is_set { 1 } else { 0 };
}

fn has_key(found: Found, key: usize) -> bool {
    found & 1 << key != 0
}

#[allow(dead_code)]
fn found_keys_str(found: Found, nkeys: usize) -> String {
    let mut indices = Vec::new();
    for i in 0 .. nkeys {
        if found & (1 << i) != 0 {
            indices.push(i);
        }
    }
    format!("{:?}", indices)
}


fn find_all_keys_(
    map: &mut Map, keys: &Keys, doors: &Doors, coords: &mut Vec<Coord>,
    found: Found, states_at: &mut StatesAt,
    steps: PathLength) -> Option<PathLength>
{
    let nk = keys.len();
    let all_keys_mask = (1 << nk) - 1;

    let mut path = None;
    for c in 0 .. coords.len() {
        //println!("--------------\nI'm at {:?} in {:?} steps with keys {:?}", c, steps, found_keys_str(found, nk));
        let keys_ = find_keys(map, keys, &coords[c], found);
        //println!("achievable keys: {:?}", keys_);

        //  save current coord
        let c_ = coords[c].clone();

        for (key_, steps_) in keys_ {
            if !has_key(found, key_) {
                //println!("I'm at {:?} in {:?} steps with keys {:?}", c, steps, found_keys_str(found, nk));

                let found = found | 1 << key_;
                //println!("go for key {:?}", key_);

                let steps = steps + steps_;

                let path_ =
                //  all keys are collected
                if found & all_keys_mask == all_keys_mask {
                    //println!("all is collected in {:?} steps", steps);
                    Some(steps)
                } else {
                    if update_state_at(nk, key_, &mut states_at[key_], found, steps) {
                        //println!("proceeding with key {:?}", key_);

                        set_door(map, &doors[key_], false);
                        coords[c] = keys[key_].clone();

                        let path_ = find_all_keys_(map, keys, doors, coords, found, states_at, steps);

                        set_door(map, &doors[key_], true);

                        path_
                    } else {
                        //println!("key {:?} already has better path", key_);
                        None
                    }
                };

                if path_.is_some() && (path.is_none() || path_.unwrap() < path.unwrap()) { path = path_; };

                //println!("back to {:?}", c);
            }
        }

        //  restore coord
        coords[c] = c_;
    }
    path
}


fn find_all_keys(map: &Map, keys: &Keys, doors: &Doors, start: Coord) -> PathLength {
    let found = 0;
    let mut states_at = vec![HashMap::new(); keys.len()];
    let mut coords = vec![start];

    find_all_keys_(&mut map.clone(), keys, doors, &mut coords, found, &mut states_at, 0).unwrap()
}


fn find_all_keys_4(map: &Map, keys: &Keys, doors: &Doors, (x, y): Coord) -> PathLength {
    let mut map = map.clone();
    map.0[y][x] = 1;
    map.0[y][x - 1] = 1;
    map.0[y][x + 1] = 1;
    map.0[y - 1][x] = 1;
    map.0[y + 1][x] = 1;

    let mut starts = vec![(x - 1, y - 1), (x + 1, y - 1), (x - 1, y + 1), (x + 1, y + 1)];
    
    let found = 0;
    let mut states_at = vec![HashMap::new(); keys.len()];

    find_all_keys_(&mut map, keys, doors, &mut starts, found, &mut states_at, 0).unwrap()
}

