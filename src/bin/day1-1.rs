use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("inputs/day1")?;
    let reader = BufReader::new(file);
    let mut list1 = vec![];
    let mut list2 = vec![];
    for line in reader.lines() {
        let line = line?;
        let (item1, item2) = line.split_once("   ").unwrap();
        list1.push(item1.parse::<u32>().unwrap());
        list2.push(item2.parse::<u32>().unwrap());
    }
    list1.sort();
    list2.sort();
    let total_distance: u32 = list1
        .into_iter()
        .zip(list2.into_iter())
        .map(|(a, b)| a.abs_diff(b))
        .sum();
    println!("{total_distance}");
    Ok(())
}
