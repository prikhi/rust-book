fn main() {
    println!("Hello, world!");
    another_function();
    another_function2(5);
    another_function3(5, 6);
    let five = five();
    println!("Five is: {}", five);
    let six = plus_one(five);
    println!("Plus one is: {}", six);
}

fn another_function() {
    println!("Another Function");
}

fn another_function2(x: i32) {
    println!("The value of x is: {}", x);
}

fn another_function3(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
