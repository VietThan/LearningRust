fn another_function(x: i32) {
    println!("The value of x is: {x}");
    // in rust 1.57 it was 
    //println!("The value of x is: {x}", x=x);
}

fn main() {
    another_function(5);
}

