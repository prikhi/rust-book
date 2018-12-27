fn main() {
    let mut x = 5;
    //let x = 5;    // re-assignment w/o let needs `mut`
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is {}", y);

    let z = 2.0; // f64
    let z_: f32 = 3.0;
    println!("{} {}", z, z_);

    another_function();
}

fn another_function() {
    println!("Another function.");
}
