use std::{collections::HashSet, fs};

use aoc2024::{get_input_file, MainResult};

fn main() -> MainResult {
    let file_path = get_input_file(8, false);
    let data = fs::read_to_string(&file_path)?;

    let data = data
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    let mut frequency_nodes = Vec::new();
    for (i, row) in data.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '.' {
                // empty space
            } else {
                // filled with something

                frequency_nodes.push(FreqNode { c: *c, i, j });
            }
        }
    }

    let mut s = HashSet::<(usize, usize)>::new();
    for (i, node) in frequency_nodes.iter().enumerate() {
        let mut did_add_node = false;
        for similar_node in frequency_nodes.iter().skip(i + 1).filter(|n| n.c == node.c) {
            // println!("check8ng {:?} w {:?}", node, similar_node);
            let diff_i = similar_node.i - node.i;
            let diff_j = similar_node.j as isize - node.j as isize;
            if node.i >= diff_i {
                let mut an1_i = node.i - diff_i;
                let mut an1_j = node.j as isize - diff_j;
                while !is_oob(an1_i, an1_j, &data) {
                    // println!("ufound an at {an1_i},{an1_j}");
                    s.insert((an1_i, an1_j as usize));
                    if an1_i >= diff_i {
                        an1_i -= diff_i;
                        an1_j -= diff_j;
                    } else {
                        break;
                    }
                }
            }
            let mut an2_i = similar_node.i + diff_i;
            let mut an2_j = similar_node.j as isize + diff_j;
            while !is_oob(an2_i, an2_j, &data) {
                // println!("dfound an at {an2_i},{an2_j}");
                s.insert((an2_i, an2_j as usize));

                an2_i += diff_i;
                an2_j += diff_j;
            }
            did_add_node = true;
            // println!("adding extra {similar_node:?}");
            s.insert((similar_node.i, similar_node.j));
        }
        if did_add_node {
            // println!("adding extra {node:?}");
            s.insert((node.i, node.j));
        }
    }

    // println!("{:?}", s);
    // println!(
    //     "{}",
    //     s.iter()
    //         .map(|n| format!("{},{}, in e: {}\n", n.0, n.1, expected[n.0][n.1]))
    //         .collect::<String>()
    // );
    println!("Count: {}", s.len());

    Ok(())
}

type Data = Vec<Vec<char>>;

fn is_oob(c1: usize, c2: isize, data: &Data) -> bool {
    c2 < 0 || c1 >= data.len() || c2 as usize >= data[0].len()
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct FreqNode {
    c: char,
    i: usize,
    j: usize,
}
