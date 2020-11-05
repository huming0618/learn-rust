fn main(){
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };

    println!("the value of y is: {}", y);
    println!("the value of x is:{}", x);
}

