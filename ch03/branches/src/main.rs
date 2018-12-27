fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    loops();

    println!(
        "32 F in C is: {}\n212 F in C is: {}",
        to_celsisus(32.0),
        to_celsisus(212.0)
    );

    println!("5th fibonacci number is: {}", gen_fib(5));
}

fn loops() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    assert_eq!(result, 20);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("Liftoff!");

    let a = [10, 20, 30, 40, 50];
    for item in a.iter() {
        println!("the value is: {}", item);
    }

    for num in (1..4).rev() {
        println!("{}!", num);
    }
    println!("Lift Off!");
}

fn to_celsisus(f: f64) -> f64 {
    (f - 32f64) / 9f64 * 5f64
}

fn gen_fib(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        gen_fib(n - 1) + gen_fib(n - 2)
    }
}
