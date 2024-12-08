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
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if data[i][j] == '.' {
                // empty space
            } else {
                // filled with something

                frequency_nodes.push(FreqNode {
                    c: data[i][j],
                    i,
                    j,
                });
            }
        }
    }

    let mut s = HashSet::new();
    for (i, node) in frequency_nodes.iter().enumerate() {
        for similar_node in frequency_nodes.iter().skip(i + 1).filter(|n| n.c == node.c) {
            // println!("check8ng {:?} w {:?}", node, similar_node);
            let diff_i = similar_node.i - node.i;
            let diff_j = similar_node.j as isize - node.j as isize;
            if node.i >= diff_i {
                let an1_i = node.i - diff_i;
                let an1_j = node.j as isize - diff_j as isize;
                if !is_oob(an1_i, an1_j, &data) {
                    // println!("ufound an at {an1_i},{an1_j}");
                    s.insert(FreqNode {
                        c: '#',
                        i: an1_i,
                        j: an1_j as usize,
                    });
                }
            }
            let an2_i = similar_node.i + diff_i;
            let an2_j = similar_node.j as isize + diff_j as isize;
            if is_oob(an2_i, an2_j, &data) {
                continue;
            }
            // println!("dfound an at {an2_i},{an2_j}");
            s.insert(FreqNode {
                c: '#',
                i: an2_i,
                j: an2_j as usize,
            });
        }
    }

    // println!("{:?}", s);
    println!("Count: {}", s.len());

    Ok(())
}

type Data = Vec<Vec<char>>;

fn is_oob(c1: usize, c2: isize, data: &Data) -> bool {
    c2 < 0 || c1 >= data.len() || c2 as usize >= data[0].len()
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct FreqNode {
    c: char,
    i: usize,
    j: usize,
}
