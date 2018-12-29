fn main() {
    value_in_cents(&Coin::Penny);
    value_in_cents(&Coin::Quarter(UsState::Maryland));

    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    };

    if let Some(3) = some_u8_value {
        println!("three");
    }

    let coin = Coin::Penny;
    let mut _count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State Quarter from {:?}!", state);
    } else {
        _count += 1;
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Maryland,
    Virginia,
    California,
    NewYork,
}

fn value_in_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State Quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(x) => Some(x + 1),
    }
}
