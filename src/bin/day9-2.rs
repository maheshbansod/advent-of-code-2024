use std::{
    collections::{HashMap, HashSet},
    fs,
};

use aoc2024::{get_input_file, MainResult};

fn main() -> MainResult {
    let file_path = get_input_file(9, false);
    let data = fs::read_to_string(file_path)?;
    let data = data.lines().next().unwrap();
    let data = data.chars().map(|c| c.to_digit(10).unwrap() as u64);

    let mut free_space = data
        .clone()
        .enumerate()
        .filter(|(i, s)| i % 2 == 1 && *s != 0)
        .collect::<Vec<_>>();
    let files = data
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        // .map(|(i, f)| (i / 2, f))
        .collect::<Vec<_>>();

    let mut moved_files = HashMap::<usize, Vec<_>>::new();

    let mut moved_hs = HashSet::new();

    for (i, flen) in files.iter().rev() {
        if let Some(space) = free_space.iter_mut().find(|(j, s)| j < i && s >= flen) {
            space.1 -= flen;
            let file_id = *i / 2;
            moved_hs.insert(file_id);
            if let Some(m) = moved_files.get_mut(&space.0) {
                m.push((space.0, i, *flen));
            } else {
                let v = vec![(space.0, i, *flen)];
                moved_files.insert(space.0, v);
            }
        }
    }

    for (_key, value) in moved_files.iter() {
        for (_space_i, i, flen) in value {
            if *flen != 0 {
                free_space.push((**i, *flen));
            }
        }
    }

    println!("{free_space:?}");
    println!("{moved_files:?}");

    let mut pos = 0;
    let mut sum = 0;
    for (i, flen) in files.iter() {
        // first let's get all the free space that's before this file
        let filtered_keys = moved_files.keys().filter(|&key| key < i);
        let mut keys_to_remove = vec![];
        for key in filtered_keys {
            let space_files = moved_files.get(key).unwrap();
            for (_, i, flen) in space_files {
                let file_id = **i / 2;
                let product = range_sum(pos, pos + flen) * file_id as u64;
                sum += product;
                println!("fi: {file_id}, p: {pos}, {flen}, prod: {product}");
                pos += flen;
            }
            // let fs = free_space.iter().find(|f| f.0 == *key).unwrap();
            // println!("free space at {key}: {}", fs.0);
            keys_to_remove.push(*key);
            // pos += fs.1;
        }
        for key in keys_to_remove {
            moved_files.remove(&key);
        }
        // let's mabe remove free space
        ;
        free_space.iter_mut().filter(|f| f.0 <= *i).for_each(|fs| {
            println!("ifree space at {i},{}: {}", fs.0, fs.1);
            pos += fs.1;
            fs.1 = 0;
            fs.0 = 100;
        });
        // now the file at i
        let file_id = i / 2;
        if moved_hs.contains(&file_id) {
            continue;
        }
        let product = range_sum(pos, pos + flen) * file_id as u64;
        println!("gfi: {file_id}, p: {pos}, fl: {flen}, prod: {product}");
        sum += product;
        pos += flen;
    }
    println!("sum: {sum}");

    Ok(())
}

fn range_sum(a: u64, b: u64) -> u64 {
    let a = if a == 0 { 0 } else { a * (a - 1) };
    let b = b * (b - 1);
    (b - a) / 2
}
