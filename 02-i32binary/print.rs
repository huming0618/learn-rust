fn main(){
    let mut val = -2147483648;
    println!("val: {:b}", val);
    println!("val: {:#032b}", val);
    
    val = 2147483647;
    println!("val: {:b}", val);
    println!("val: {:#032b}", val);
    
    let flags = 0b0000000000101100i32;
    println!("flags: {:#018b}", flags);
}