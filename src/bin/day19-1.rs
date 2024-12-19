use std::fs;

use aoc2024::{get_input_file, MainResult};

fn main() -> MainResult {
    let file_path = get_input_file(19, false);
    let data = fs::read_to_string(&file_path)?;
    let (patterns, designs) = {
        let (patterns, designs) = data.split_once("\n\n").unwrap();
        let patterns = patterns.split(", ").collect::<Vec<_>>();
        let designs = designs.lines().collect::<Vec<_>>();
        (patterns, designs)
    };
    let mut count = 0;
    for design in designs {
        let mut to_try: Vec<&str> = vec![design];
        let mut done = false;
        while let Some(design) = to_try.pop() {
            if design.is_empty() {
                done = true;
                break;
            }
            for dp in patterns
                .iter()
                .filter(|&p| design.starts_with(p))
                .map(|p| &design[p.len()..])
            {
                to_try.push(dp);
            }
        }
        if done {
            count += 1;
        }
    }
    println!("count: {count}");
    Ok(())
}
