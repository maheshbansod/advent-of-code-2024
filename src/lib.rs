pub mod maps;
pub mod math;

use std::error::Error;

pub fn get_input_file(day: u32, use_test: bool) -> String {
    if use_test {
        format!("inputs/day{day}-test")
    } else {
        format!("inputs/day{day}")
    }
}

pub type MainResult = Result<(), Box<dyn Error>>;
