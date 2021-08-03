

fn main(){
   assert_eq!( find_median_sorted_arrays(vec![1,2,3], vec![4,5]),3.0); 
     assert_eq!( find_median_sorted_arrays(vec![1,2,3], vec![4,5,6]),3.5); 
}

// O(log(M+N))
//转化为topk
// 2k=(len(l1)+len(l2))/2
//如果 l1[k]=l2[k]则　中位数为l1[k]
// 如果 l1[k]<=l2[k]则 l1[0]必定不可能为第2k个数
// 转化为l1[k+1 ...] 和l2[0...]中第k小的数
//当k=1或一个数组为空时停止
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64{
    let k=nums1.len()+nums2.len();
    if k&1==1{
        return findkth(&nums1, &nums2, k/2+1) as f64;
    }else {
        let small=findkth(&nums1, &nums2, k/2+1);
        let large =findkth(&nums1, &nums2, k>>1);
        return (small+large)as f64 /2.0 ;
    }

}

fn findkth(nums1: &[i32],nums2: &[i32],k:usize)->i32{
    if nums1.len()==0{
        return nums2[k-1];
    } 
    if nums2.len()==0{
        return   nums1[k-1];
    }
    if k==1{
        return  i32::min(nums1[0], nums2[0]);
    }
    
    let a = if nums1.len()>=k/2{
        Some(nums1[k/2-1])
    }else{
        None
    };
    let b =if nums2.len()>=k/2{
        Some(nums2[k/2-1])
    }else{
        None
    };

    if b.is_none() || a.is_some()&& a.unwrap()<b.unwrap(){
        return findkth(&nums1[k>>1..], &nums2, k-k/2)
    }
        findkth(&nums1, &nums2[k>>1..], k-k/2)
}
// O(m+n)
// fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
//     let cnt=nums1.len()+nums2.len();
//     let mut v=Vec::new();
//     v.resize(cnt, 0);
//     let mut i=0;
//     let mut j=0;
    
//     for  num in v.iter_mut(){
//         if i==nums1.len(){
//             *num=nums2[j];
//             j+=1;
//             continue;
//         }else if j==nums2.len() {
//             *num=nums1[i];
//             i+=1;
//             continue;
//         }
//         if nums1[i]< nums2[j]{
//             *num=nums1[i];
//             i+=1;
//         }else{
//             *num=nums2[j];
//             j+=1;
//         }
//     }
        
//     if v.len()&1==1{
//         return v[v.len()>>1] as f64
//     }else {
//         return (v[v.len()>>1] as f64 +  v[(v.len()>>1)-1] as f64)* 0.5
//     }

// }