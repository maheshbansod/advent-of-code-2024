use std::{
    collections::{HashMap, HashSet},
    fs,
};

use aoc2024::{get_input_file, MainResult};

fn main() -> MainResult {
    let file_path = get_input_file(5, false);
    let data = fs::read_to_string(&file_path)?;

    let (rules, updates) = data.split_once("\n\n").unwrap();

    let rules = generate_rules(rules);
    let mut valid_mids = vec![];

    for update in updates.lines() {
        let update = update.split(',').map(|n| n.parse::<u32>().unwrap());
        let mut is_invalid = false;
        let mut update_numbers = update.clone().rev();
        while let Some(page) = update_numbers.next() {
            if let Some(rule_data) = rules.get(&page) {
                let it_clone = update_numbers.clone();
                for p2 in it_clone {
                    if rule_data.contains(&p2) {
                        is_invalid = true;
                        break;
                    }
                }
                if is_invalid {
                    break;
                }
            }
        }
        if !is_invalid {
            let updates = update.collect::<Vec<_>>();
            let mid = updates[updates.len() / 2];
            valid_mids.push(mid);
        }
    }
    println!("sum: {}", valid_mids.iter().sum::<u32>());

    Ok(())
}

fn generate_rules(rules: &str) -> HashMap<u32, HashSet<u32>> {
    let lines = rules.lines();
    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();
    for rule in lines {
        let (a, b) = rule.split_once("|").unwrap();
        let a = a.parse().unwrap();
        let b = b.parse().unwrap();
        if let Some(r) = rules.get_mut(&a) {
            r.insert(b);
        } else {
            let mut set = HashSet::new();
            set.insert(b);
            rules.insert(a, set);
        }
    }
    rules
}