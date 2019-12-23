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
type Found = Vec<bool>;
type StateAt = HashMap<Found, PathLength>;
type StatesAt = Vec<StateAt>;


fn main() {
    let (mut map, keys, doors, start) = parse("18_3.input");
    //println!("{:?}", map);
    //println!("{:?}", keys);
    //println!("{:?}", doors);
    //println!("{:?}", start);

    //  part 1
    let steps = find_all_keys(&mut map, &keys, &doors, start);
    println!("{}", steps);

    //  part 2
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


fn find_all_keys(map: &mut Map, keys: &Keys, doors: &Doors, start: Coord) -> PathLength {
    let mut found = vec![false; keys.len()];
    let mut states_at = vec![HashMap::new(); keys.len()];

    find_all_keys_(map, keys, doors, start, &mut found, &mut states_at, 0).unwrap()
}


fn find_all_keys_(
    map: &mut Map, keys: &Keys, doors: &Doors, c: Coord,
    found: &mut Found, states_at: &mut StatesAt,
    steps: PathLength) -> Option<PathLength>
{
    fn is_superset(f1: &Found, f2: &Found) -> bool {
        let mut is = true;
        for i in 0 .. f1.len() {
            if !f1[i] && f2[i] {
                is = false;
                break;
            }
        }
        is
    }

    fn update_state_at(key: usize, state_at: &mut StateAt, found: &Found, steps: PathLength) -> bool {
        let has_better = state_at.iter()
            .any(|(found_, steps_)| is_superset(found_, found) && *steps_ <= steps);
        if !has_better {
            state_at.retain(|found_, steps_| !(is_superset(found, found_) && steps <= *steps_));
            assert!(!state_at.contains_key(found));
            state_at.insert(found.clone(), steps);
            //println!("Added state for key {:?}: {:?} => {:?}", key, found_keys_str(found), steps);
            //println!("All states for key {:?}: {:?}", key, state_at);
        }
        !has_better
    }

    fn found_keys_str(found: &Found) -> String {
        let mut indices = Vec::new();
        for i in 0 .. found.len() {
            if found[i] {
                indices.push(i);
            }
        }
        format!("{:?}", indices)
    }

    fn set_door((map, _, _): &mut Map, (x, y): &Coord, is_set: bool) {
        map[*y][*x] = if is_set { 1 } else { 0 };
    }


    //println!("--------------\nI'm at {:?} in {:?} steps with keys {:?}", c, steps, found_keys_str(found));
    let keys_ = find_keys(map, keys, &c, found);
    //println!("achievable keys: {:?}", keys_);

    let mut path = None;
    for (key_, steps_) in keys_ {
        if !found[key_] {
            //println!("I'm at {:?} in {:?} steps with keys {:?}", c, steps, found_keys_str(found));

            found[key_] = true;
            //println!("go for key {:?}", key_);

            let steps = steps + steps_;

            let path_ =
            //  all keys are collected
            if found.iter().all(|k| *k) {
                println!("all is collected in {:?} steps", steps);
                Some(steps)
            } else {
                if update_state_at(key_, &mut states_at[key_], found, steps) {
                    //println!("proceeding with key {:?}", key_);

                    set_door(map, &doors[key_], false);

                    let path_ = find_all_keys_(map, keys, doors, keys[key_].clone(), found, states_at, steps);

                    set_door(map, &doors[key_], true);

                    path_
                } else {
                    //println!("key {:?} already has better path", key_);
                    None
                }
            };

            if path_.is_some() && (path.is_none() || path_.unwrap() < path.unwrap()) { path = path_; };
            found[key_] = false;

            //println!("back to {:?}", c);
        }
    }
    path
}


fn find_keys((map, w, h): &Map, keys: &Keys, (x, y): &Coord, found: &Found) -> Vec<(usize, PathLength)> {

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
            if key.is_some() && !found[key.unwrap()] { keys_.push((key.unwrap(), d)); }

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

