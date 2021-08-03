
fn main(){
    let nums=vec![3,3];
    let target=6;
    println!("{:?}",two_sum(nums, target));
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut rst=vec![];
    let mut hs:HashMap<i32,usize>=HashMap::new();
    for   (idx,num) in nums.iter().enumerate(){
        if let Some(a)=hs.get(&(target-*num)){
                rst.push(*a as i32);
                rst.push(idx as i32);
                break;
        } else{
            hs.insert(*num, idx);
        }
    }
    rst
       
}