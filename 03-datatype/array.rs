fn main(){
    let a: [i32;5]= [3;5];
    let test = &a[..];
    println!("{:?}", a);
    println!("{:?}", test);

}
