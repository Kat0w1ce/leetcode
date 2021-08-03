
fn main(){
    assert_eq!(length_of_longest_substring(String::from("abcabcbb")),3)
}

fn length_of_longest_substring(s: String) -> i32 {
        let mut len: i32 = 0;
        let mut start_idx: i32 = 0;
        let mut last_idx: i32;
        let mut index: [i32; 256] = [-1; 256];

        for (i, c) in s.into_bytes().iter().enumerate() {
            last_idx = index[(*c) as usize];
            if last_idx > -1 {
                start_idx = i32::max(start_idx, last_idx + 1);
            }
            len = i32::max(len, i as i32 - start_idx + 1);
            index[*c as usize] = i as i32;
        }
        len

    }

// my solution
// fn length_of_longest_substring(s: String) -> i32 {
//     let mut sbegin=0;
//     let mut send:usize=0;
//     let mut v:Vec<i32>= vec![];
//     v.resize(256, 0);
//     let mut rst=0;
//     let mut len=0;
//     let ps=s.as_bytes();
//     while send<ps.len() {
//         if v[ ps[send] as usize]==0{
//             v[ps[send] as usize]=1;
//             send+=1;
//             len+=1
//         }else {
//             rst=max(rst, len);
//             len-=1;
//             v[ps[sbegin] as usize]-=1;
//             sbegin+=1;
//         }
        
//     }
    
//     max(rst,len)
// }

//abcaaddd