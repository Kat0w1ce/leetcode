fn main() {}

fn length_of_last_word(s: String) -> i32 {
    let a: Vec<&str> = s.split_whitespace().collect();
    a[a.len() - 1].len() as i32
}
