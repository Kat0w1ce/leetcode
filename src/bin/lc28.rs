pub fn str_str(haystack: String, needle: String) -> i32 {
    // api
    // haystack.find(needle).map(|x| x as i32).unwrap_or(-1);
    match (haystack.is_empty(), needle.is_empty()) {
        (true, true) => return 0,
        (true, false) => return 0,
        (false, true) => return -1,
        _ => (),
    }
    if haystack.len() < needle.len() {
        return -1;
    }
    let a = haystack.as_bytes();
    let b = needle.as_bytes();
    let mut rst = -1;
    for i in 0..a.len() {
        if a[i] == b[0] {
            let mut flag = true;
            for j in 0..b.len() {
                if i + j >= a.len() || a[i + j] != b[j] {
                    flag = false;
                }
            }
            if flag {
                rst = i as i32;
                break;
            }
        }
    }

    rst as i32
}

fn main() {}
