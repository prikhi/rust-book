fn main() {
    let _v: Vec<i32> = Vec::new();
    let macro_v = vec![1, 2, 3, 4, 5];
    let mut v2 = Vec::new();

    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);

    let third: &i32 = &macro_v[2];
    println!("The third element is {}", third);

    match macro_v.get(2) {
        None => println!("There is no third element."),
        Some(third) => println!("The third element is {}", third),
    }

    for i in &macro_v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ]; // row : Vec<SpreadsheetCell>
    for i in row {
        println!("{:?}", i);
    }

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
