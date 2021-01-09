//求和
fn sum(list:&[u32])-> Option<u32> {
    //抛异常
    if list.len() == 0 {
        return None;
    }
    let t:u32=list.iter().sum();
    Some(t)
}


fn main() {
    let demo_list =  vec![1,2,3];
    let res = sum(&demo_list[..]);
    println!("the sum is {:?}", res); 
}