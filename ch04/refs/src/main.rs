use std::option::Option;
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("hello");
    change(&mut s2);

    println!("1st: {}", first_word(&s2));
    let my_string = "Hello   world, this has more than 2 words";
    second_word(&my_string).map(|s| {
        println!("2nd: {}", s);
    });
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}

fn first_word(s: &str) -> &str {
    let bytestring = s.as_bytes();
    for (i, &item) in bytestring.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn second_word(s: &str) -> Option<&str> {
    let bytestring = s.as_bytes();
    let mut passed_first = false;
    let mut in_break = false;
    let mut first_index = None;
    let mut second_index = None;
    for (i, &item) in bytestring.iter().enumerate() {
        if item == b' ' {
            if !passed_first {
                passed_first = true;
                in_break = true;
            }
            if !in_break {
                second_index = Some(i);
                break;
            }
        } else if in_break {
            in_break = false;
            first_index = Some(i);
        }
    }

    first_index.and_then(|first| match second_index {
        None => Some(&s[first..]),
        Some(second) => Some(&s[first..second]),
    })
}
