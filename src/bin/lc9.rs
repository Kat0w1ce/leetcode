
fn main(){

}
pub fn is_palindrome(x: i32) -> bool {
    if x<0 {
        return false
    }
    if x<10 {
        return  true
    }
    let s=x.to_string().into_bytes();
    let mut i=0;
    let mut j=s.len()-1;
    while i<j {
        if s[i]!=s[j]{
            return  false;
        }
        i+=1;
        j-=1;
        
    }
    true
        
}