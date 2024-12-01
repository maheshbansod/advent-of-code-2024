use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("inputs/day1")?;
    let reader = BufReader::new(file);
    let mut list1 = vec![];
    let mut list2 = vec![];
    let mut frequency = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        let (item1, item2) = line.split_once("   ").unwrap();
        let item1 = item1.parse::<u32>().unwrap();
        list1.push(item1);
        let item2 = item2.parse::<u32>().unwrap();
        list2.push(item2);
        if let Some(item2_frequency) = frequency.get_mut(&item2) {
            *item2_frequency += 1;
        } else {
            frequency.insert(item2, 1);
        }
    }
    let similarity_score: u32 = list1
        .iter()
        .map(|item| {
            if let Some(f) = frequency.get(item) {
                item * f
            } else {
                0
            }
        })
        .sum();
    println!("{similarity_score}");
    Ok(())
}
