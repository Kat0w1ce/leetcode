fn main() {
    println!("{}", climb_stairs(44))
}

pub fn climb_stairs(n: i32) -> i32 {
    let mut v = vec![];
    v.resize(46, 0);
    v[1] = 1;
    v[2] = 2;
    for i in 3..(n + 1) as usize {
        v[i] = v[i - 1] + v[i - 2];
    }
    v[n as usize]
}
