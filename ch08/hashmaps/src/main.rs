mod creation {
    use std::collections::HashMap;
    pub fn main() {
        let mut scores = HashMap::new();
        // these calls inform type-checker that scores is String->i32
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        // can also use .collect method on vector of tuples
        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];
        let teams_and_scores = teams.iter().zip(initial_scores.iter());
        // could use HashMap<_,_> here and let vectors inform hashmap type
        let _scores: HashMap<&String, &i32> = teams_and_scores.collect();
    }
}

mod ownership {
    use std::collections::HashMap;
    pub fn main() {
        let field_name = String::from("favorite color");
        let field_value = String::from("blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        // Maps take ownership of keys & values so the follow is invalid:
        //let _compiler_error = field_name + &field_value;

        // note: inserting references won't move the values, so the references
        // must be valid for as long as the hashmap is valid - see chapter 10.
    }
}

mod accessing {
    use std::collections::HashMap;
    pub fn main() {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_name = String::from("Blue");
        let _team_score = scores.get(&team_name); // has type Option<&i32>

        // iteration - order is arbitrary
        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }
    }
}

mod updating {
    use std::collections::HashMap;
    pub fn main() {
        let mut scores = HashMap::new();
        let blue_team = String::from("Blue");
        scores.insert(blue_team.clone(), 10);
        // overwrite the value for Blue
        scores.insert(blue_team.clone(), 25);
        println!("{:#?}", scores);

        // entry & or_insert allow us to insert only if key doesn't exist
        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(blue_team).or_insert(50);
        println!("{:#?}", scores);

        // we can also update a value using the old value
        let text = "hello world wonderful world";
        let mut text_map = HashMap::new();
        // build the map, incrementing the count using the returned mutable ref
        for word in text.split_whitespace() {
            let count = text_map.entry(word).or_insert(0);
            *count += 1;
        }
        println!("{:#?}", text_map);
    }
}

// end of chapter exercises
mod exercises {
    pub fn main() {
        let ints = vec![1, 2, 3, 4, 5, 6, 7, 7, 7];
        ex1(&ints);
        println!("ex2: {}", ex2("hello world apples"));
    }
    fn ex1(ints: &Vec<i32>) {
        // average
        {
            let avg: f32;
            let mut sum = 0;
            for i in ints {
                sum += i;
            }
            avg = sum as f32 / ints.len() as f32;
            println!("Avg: {}", avg);
        }

        // median
        {
            let mut sorted_ints = ints.clone();
            sorted_ints.sort();
            let vec_length = sorted_ints.iter().len();
            match sorted_ints.get(vec_length / 2) {
                None => println!("No median, empty list passed!"),
                Some(median) => println!("Median: {}", median),
            }
        }

        // mode
        {
            use std::collections::HashMap;
            let mut int_map = HashMap::new();
            // insert w/ counts
            for i in ints {
                let count = int_map.entry(i).or_insert(0);
                *count += 1;
            }
            let mut mode = None;
            for (key, count) in int_map {
                match mode {
                    None => mode = Some((key, count)),
                    Some((current_mode, current_count)) => {
                        if current_count < count {
                            mode = Some((key, count))
                        } else {
                            mode = Some((current_mode, current_count))
                        }
                    }
                }
            }
            match mode {
                None => println!("No mode, empty list passed!"),
                Some((m, _)) => println!("Mode: {}", m),
            }
        }
    }

    fn ex2(initial: &str) -> String {
        let mut result = String::new();

        let mut in_word = false;
        let mut word_ending = String::new();
        for c in initial.chars() {
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    if !in_word {
                        word_ending = String::from("-hay");
                    }
                    result.push_str(&c.to_string());
                    in_word = true;
                }
                ' ' => {
                    if in_word {
                        result.push_str(&word_ending)
                    }
                    in_word = false;
                    result.push_str(&c.to_string());
                }
                _ => {
                    if !in_word {
                        word_ending = format!("-{}ay", &c);
                    } else {
                        result.push_str(&c.to_string());
                    }
                    in_word = true;
                }
            }
        }
        if in_word {
            result.push_str(&word_ending);
        }

        result
    }
}

fn main() {
    creation::main();
    ownership::main();
    accessing::main();
    updating::main();
    exercises::main();
}
