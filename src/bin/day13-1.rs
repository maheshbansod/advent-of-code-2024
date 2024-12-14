use aoc2024::math::egcd;
use std::fs;

use aoc2024::{get_input_file, MainResult};

fn main() -> MainResult {
    let file_path = get_input_file(13, false);
    let data = fs::read_to_string(&file_path)?;
    let data = data
        .split("\n\n")
        .map(|m| {
            let mut lines = m.lines();
            let a_button = parse_button(lines.next().unwrap());
            let b_button = parse_button(lines.next().unwrap());
            let prize_location = parse_prize(lines.next().unwrap());
            Machine::new(prize_location, a_button, b_button)
        })
        .collect::<Vec<_>>();
    let mut sum = 0;
    for machine in data {
        let a1 = machine.a_button.0;
        let b1 = machine.b_button.0;
        let c1 = machine.prize_location.0;
        let a2 = machine.a_button.1;
        let b2 = machine.b_button.1;
        let c2 = machine.prize_location.1;
        // above are the constants we workin with
        // i have to find integers x and y where
        // a1x + b1y = c1
        // a2x + b2y = c2
        // x is the amount of times we need to press the a button
        // y is the amount of times we need to press the b button

        let (g1, x10, y10) = egcd(a1, b1);
        if c1 % g1 != 0 {
            // no solution
            // println!("{a1},{b1},{c1},{g1}");
            // panic!("no solution")
            continue;
        }
        let (g2, x20, y20) = egcd(a2, b2);
        if c2 % g2 != 0 {
            // no solution
            // println!("{a2},{b2},{c2},{g2}");
            // panic!("no solution")
            continue;
        }
        // particular solutions
        let x10 = x10 * c1 / g1;
        let y10 = y10 * c1 / g1;
        let x20 = x20 * c2 / g2;
        let y20 = y20 * c2 / g2;
        let d = (b1 / g1) * (-a2 / g2) - (-b2 / g2) * (a1 / g1);
        // let dk1 = (x20 - x10) * (-a2 / g2) - (-b2 / g2) * (y10 - y20);
        let dk2 = (b1 / g1) * (y10 - y20) - (x20 - x10) * (a1 / g1);
        if d == 0 {
            panic!("no solulu");
        } else if dk2 % d != 0 {
            continue;
        }
        // let k1 = dk1 / d;
        let k2 = dk2 / d;
        // println!("new solulu");
        // let x = x10 + k1 * b1 / g1;
        // let y = y10 - k1 * a1 / g1;
        // println!("1: k1: {k1}, x {x},y:{y}, c: {}", x * a1 + y * b1);
        let x = x20 + k2 * b2 / g2;
        let y = y20 - k2 * a2 / g2;
        if x < 0 || y < 0 {
            continue;
        }
        // println!("2: k2: {k2}, x {x},y:{y}, c: {}", x * a2 + y * b2);
        sum += x * 3 + y;
    }
    println!("sum: {sum}");
    Ok(())
}

#[derive(Debug)]
struct Machine {
    prize_location: Coord,
    a_button: Coord,
    b_button: Coord,
}
type Coord = (i32, i32);

impl Machine {
    fn new(prize_location: Coord, a_button: Coord, b_button: Coord) -> Self {
        Self {
            prize_location,
            a_button,
            b_button,
        }
    }
}

fn parse_button(s: &str) -> Coord {
    let mut c = s[10..].split(", ").map(|s| s[2..].parse().unwrap());
    (c.next().unwrap(), c.next().unwrap())
}

fn parse_prize(s: &str) -> Coord {
    let mut c = s[7..].split(", ").map(|s| s[2..].parse().unwrap());
    (c.next().unwrap(), c.next().unwrap())
}
