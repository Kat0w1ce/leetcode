fn main() {}

pub fn set_zeroes2(matrix: &mut Vec<Vec<i32>>) {}

//space complexity O(M+N) not good
pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let mut row = vec![];
    let mut col = vec![];

    for (i, v) in matrix.iter().enumerate() {
        for (j, num) in v.iter().enumerate() {
            let mut set_row = false;
            match (num == &0, set_row) {
                (true, false) => {
                    set_row = true;
                    row.push(i);
                    col.push(j);
                }
                (true, true) => col.push(j),
                _ => (),
            }
        }
    }
    for i in row {
        for i in matrix[i].iter_mut() {
            *i = 0;
        }
    }
    for j in col {
        for v in matrix.iter_mut() {
            v[j] = 0;
        }
    }
}
