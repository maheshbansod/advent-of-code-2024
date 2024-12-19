use std::{collections::HashMap, fs};

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
    let mut solver = Solver::new(patterns);
    let count: usize = designs.iter().map(|design| solver.solve(design)).sum();
    println!("count: {count}");
    Ok(())
}

struct Solver<'a> {
    patterns: Vec<&'a str>,
    cache: HashMap<&'a str, usize>,
}

impl<'a> Solver<'a> {
    fn new(patterns: Vec<&'a str>) -> Self {
        Self {
            patterns,
            cache: HashMap::new(),
        }
    }
    fn solve(&mut self, design: &'a str) -> usize {
        if design.is_empty() {
            return 1;
        }
        self.cache.get(design).copied().unwrap_or_else(|| {
            let total_count = self
                .patterns
                .iter()
                .filter(|&p| design.starts_with(*p))
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .iter()
                .map(|p| self.solve(&design[p.len()..]))
                .sum();
            self.cache.insert(design, total_count);
            total_count
        })
    }
}
