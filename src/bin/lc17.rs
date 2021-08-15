use std::{cell::RefCell, collections::binary_heap::Iter};

fn main() {
    println!("{:?}", letter_combinations(String::from("23")))
}

pub fn letter_combinations(digits: String) -> Vec<String> {
    // use std::collections::HashMap;
    // let rst = vec![];
    let map = vec![
        vec![],
        vec![],
        vec!['a', 'b', 'c'],
        vec!['d', 'e', 'f'],
        vec!['g', 'h', 'i'],
        vec!['j', 'k', 'l'],
        vec!['m', 'n', 'o'],
        vec!['p', 'q', 'r', 's'],
        vec!['t', 'u', 'v'],
        vec!['w', 'x', 'y', 'z'],
    ];
    // let mut viter = Vec::new();
    digits.as_bytes().iter().fold(
        if digits.is_empty() {
            Vec::new()
        } else {
            vec![String::new()]
        },
        |acc, &x| {
            acc.iter()
                .flat_map(|s| {
                    std::iter::repeat(s)
                        .zip(map[(x - b'0') as usize].clone())
                        .map(|(s, b)| s.chars().chain(std::iter::once(b as char)).collect())
                        .collect::<Vec<_>>()
                })
                .collect()
        },
    )
}
