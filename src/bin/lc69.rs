use std::collections::BinaryHeap;

fn main() {
    println!("{}", my_sqrt(8))
}

pub fn my_sqrt(x: i32) -> i32 {
    bsearch(0, x, x as i64)
}

fn bsearch(start: i32, end: i32, target: i64) -> i32 {
    // let target = target as i64;
    let i = (start + end) >> 1;
    let j = i + 1;
    let ii = i as i64 * i as i64;
    let jj = j as i64 * j as i64;
    match (ii.cmp(&target), jj.cmp(&target)) {
        (std::cmp::Ordering::Equal, _) => i,
        (_, std::cmp::Ordering::Equal) => j,
        (std::cmp::Ordering::Greater, _) => bsearch(start, i, target),
        (_, std::cmp::Ordering::Less) => bsearch(j, end, target),
        _ => i,
    }
}
