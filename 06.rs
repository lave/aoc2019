use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use std::fs::File;

type Orbit = (String, String);
type System = HashMap<String, Vec<String>>;


fn main() {
    let orbits = parse("06.input");
    let system = build(orbits);
    //println!("{:?}", orbits);

    //  part 1
    let c = orbits_count(&system, "COM", 0);
    println!("{}", c);

    //  part 2
    let p = find_path(&system, "YOU", "SAN");
    println!("{}", p);
}


fn parse(filename: &str) -> Vec<Orbit> {
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);
    file.lines().map(| l | {
        let l = l.unwrap();
        let mut parts = l.split(')');
        (String::from(parts.next().unwrap()), String::from(parts.next().unwrap()))
    }).collect()
}


fn build(orbits: Vec<Orbit>) -> System {
    let mut system = HashMap::new();
    for (planet1, planet2) in orbits {
        match system.get_mut(&planet1) {
            None => { system.insert(planet1, vec![planet2]); },
            Some(planets) => { planets.push(planet2); }
        }
    }
    system
}

fn orbits_count(system: &System, planet: &str, depth: u32) -> u32 {
    let sub_count = match system.get(planet) {
        None => 0,
        Some(planets) => planets.iter()
            .map(|p| orbits_count(system, p, depth + 1))
            .sum()
    };
    depth + sub_count
}

fn find_path(system: &System, planet1: &str, planet2: &str) -> u32 {
    let p1 = find_path_(system, planet1, "COM").unwrap();
    let p2 = find_path_(system, planet2, "COM").unwrap();
    //println!("{:?}", p1);
    //println!("{:?}", p2);

    let n1 = p1.len();
    let n2 = p2.len();
    let mut i = 1;
    while p1[n1 - i] == p2[n2 - i] {
        i += 1;
    }

    ((n1 - i) + (n2 - i) + 2) as u32
}

fn find_path_<'a, 'b>(system: &'a System, to: &'b str, from: &'a str) -> Option<Vec<&'a str>> {
    if from == to {
        Some(Vec::new())
    } else {
        match system.get(from) {
            None => None,
            Some(planets) => {
                let path = planets.iter()
                    .map(|p| find_path_(system, to, p))
                    .find(|x| x.is_some())
                    .unwrap_or_default();
                match path {
                    None => None,
                    Some(mut path) => { path.push(from); Some(path) }
                }
            }
        }
    }
}

