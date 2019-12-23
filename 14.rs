use std::collections::HashSet;
use std::collections::HashMap;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

mod common;
use common::*;

type Qty = u64;
type Chemical = (String, Qty);
type Reaction = (Vec<Chemical>, Chemical);
type Reactions = HashMap<String, Reaction>;
//  all intermediate ingridients needed for a certain chemical
type Ingridients = HashSet<String>;
type AllIngridients = HashMap<String, HashSet<String>>;



fn main() {
    let reactions = parse("14.input");
    //println!("{:?}", reactions);

    //  part 1
    let ore = get_ore_for_fuel(&reactions, 1);
    println!("{}", ore);

    //  part 2
    let fuel = get_fuel_for_ore(&reactions, ore, 1_000_000_000_000);
    println!("{}", fuel);
}


fn parse(filename: &str) -> Reactions {
    let f = File::open(filename).unwrap();
    let lines = BufReader::new(&f).lines();
    lines
        .map(|line| parse_reaction(&line.unwrap()))
        .map(|r| ((r.1).0.clone(), r))
        .collect()
}

fn parse_reaction(line: &str) -> Reaction {
    let (left, right) = split_at_pattern(line, "=>");
    let inputs = left.split(',').map(|s| parse_chemical(s)).collect();
    let output = parse_chemical(right);
    (inputs, output)
}

fn parse_chemical(s: &str) -> Chemical {
    let s = s.trim();
    let (qty, name) = split_at_pattern(s, " ");
    assert!(name.chars().all(|c| c >= 'A' && c <= 'Z'), "invalid chemical name {}", name);
    (String::from(name), to_u32(qty) as Qty)
}


fn resolve_ingridients(reactions: &Reactions) -> AllIngridients {
    let mut ingridients = AllIngridients::new();
    for (name, _) in reactions {
        get_ingridients(reactions, &name, &mut ingridients);
    }
    ingridients
}

fn get_ingridients<'a, 'b>(
    reactions: &'a Reactions, name: &'a String,
    ingridients: &'b mut AllIngridients) -> &'b Ingridients
{
    if ingridients.contains_key(name) {
        ingridients.get(name).unwrap()
    } else {
        let mut all = HashSet::new();
        let reaction = reactions.get(name);
        if reaction.is_some() {
            let (inputs, _) = reaction.unwrap();
            let names = inputs.iter().map(|(name, _)| name.clone()).collect::<HashSet<_>>();
            for name in &names {
                let ins = get_ingridients(reactions, &name, ingridients);
                for n in ins {
                    all.insert(n.clone());
                }
            }
            for n in names {
                all.insert(n.clone());
            }
        }
        ingridients.insert(name.clone(), all);
        ingridients.get(name).unwrap()
    }
}


fn get_ore_for_fuel(reactions: &Reactions, fuel_qty: Qty) -> Qty {
    let ingridients = resolve_ingridients(&reactions);
    //println!("{:?}", ingridients);

    let mut what_ = HashMap::new();
    what_.insert("FUEL".to_string(), fuel_qty);
    let ingridients = get_ingridient_fors_(reactions, &ingridients, what_);

    assert!(ingridients.len() == 1);
    assert!(ingridients[0].0 == "ORE");
    ingridients[0].1
}


fn get_ingridient_fors_(
    reactions: &Reactions, ingridients: &AllIngridients,
    mut chemicals: HashMap<String, Qty>) -> Vec<Chemical>
{
    loop {
        let chem = chemicals.keys().find(|c|
            reactions.contains_key(*c) &&
            chemicals.keys().all(|c_| !ingridients.get(c_).unwrap().contains(*c)));

        //  no more ingridients which are not required to obtain others
        if chem.is_none() { break; }
        let chem = chem.unwrap().clone();

        let qty = chemicals.remove(&chem).unwrap();

        //println!("produce {} of {}", qty, chem);

        let (inputs, (_, qty_)) = reactions.get(&chem).unwrap();
        let qty_ = *qty_ as Qty;

        //  factor for the reaction - how many times we need to run it in order to get required
        //  amount of chemical
        let k = (qty + qty_ - 1) / qty_;

        //println!("will run reaction {} times, inputs are {:?}", k, inputs);

        for (n, q) in inputs {
            *chemicals.entry(n.clone()).or_insert(0) += q * k;
        }

        //println!("{:?}", chemicals);
    }
    chemicals.drain().collect()
}


fn get_fuel_for_ore(reactions: &Reactions, ore_per_fuel: Qty, ore_qty: Qty) -> Qty {
    //  do rough guess by dividing ore amount by ore/fuel ratio, and
    //  then improve it by calculating real amount of ore needed for this
    //  amount of fuel, taking delta and doing same rough guess of how many
    //  additional fuel we can get from extra ore
    let mut fuel_qty = ore_qty / ore_per_fuel;
    loop {
        let ore_qty_real = get_ore_for_fuel(reactions, fuel_qty);
        //println!("for {} fuel required {} ore", fuel_qty, ore_qty_real);
        let fuel_qty_d = (ore_qty - ore_qty_real) / ore_per_fuel;
        if fuel_qty_d == 0 { break };
        fuel_qty += fuel_qty_d;
    }

    //  try increasing fuel by 1 - I'm not sure apprximation logic above always
    //  finds exact maximum
    loop {
        let ore_qty_real = get_ore_for_fuel(reactions, fuel_qty + 1);
        if ore_qty_real > ore_qty { break; }
        fuel_qty += 1;
        //println!("for {} fuel required {} ore", fuel_qty, ore_qty_real);
    }
    fuel_qty
}
