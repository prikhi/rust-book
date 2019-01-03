mod creation {
    pub fn main() {
        let mut _new_s = String::new();
        let string_literal = "initial contents";
        let _string_from_literal = string_literal.to_string();
        let _string_literal_direct = "initial contents".to_string();
        let _using_from_method = String::from("initial contents");
        let _with_utf8_chars = String::from("السلام عليكم");
    }
}

mod updating {
    pub fn main() {
        let mut concat_string = String::from("foo");
        let concatted_string = "bar";
        concat_string.push_str(concatted_string);
        println!("s2 is {}", concatted_string);

        let mut push_char_string = String::from("lo");
        push_char_string.push('l'); // now "lol"

        let concat_with_plus = String::from("Hello, ");
        let concat_with_plus2 = String::from("World!");
        let _concatted_plus = concat_with_plus + &concat_with_plus2;
        // note: `concat_with_plus` has been moved by `+` & is now invalid
        // `+` w/ strings == fn add(self, s: &str) -> String
        // also, compiler coerces the &String into &str via `def coercian`,
        //  which turns &s2 into &s2[..]

        let tic = String::from("tic");
        let tac = String::from("tac");
        let toe = String::from("toe");
        println!("{}", tic + "-" + &tac + "-" + &toe); // tic is moved
        let tic = String::from("tic"); // so we redefine here
                                       // format! doesn't move tic
        let better_concat = format!("{}-{}-{}", tic, tac, toe);
        println!("{}", better_concat);
    }
}

mod indexing {
    pub fn main() {
        let _unindexable = String::from("hello");
        // fails because String doesn't implement Index<integer> trait
        // let h = _unindexable[0];

        assert_eq!(String::from("Hola").len(), 4);
        // this has 12 characters but takes up 24 bytes
        let unexpected_len = String::from("Здравствуйте").len();
        assert_eq!(unexpected_len == 12, false);
        assert_eq!(unexpected_len, 24);

        let russian_hello = String::from("Здравствуйте");
        // &str slices are bytes
        let russian_substr = &russian_hello[..4];
        // since each character is 2 bytes, the sub string is 2 characters
        assert_eq!(russian_substr, "Зд");
        // this will panic since we're splitting a character
        //let s = &russian_hello[0..1];
    }
}

mod iteration {
    pub fn main() {
        let russian_hello = "Здравствуйте";
        // conversion to characters is possible
        for c in russian_hello.chars() {
            println!("{}", c);
        }
        // bytes() gives raw byte ints
        for b in russian_hello.bytes() {
            println!("{}", b);
        }
    }
}

fn main() {
    creation::main();
    updating::main();
    indexing::main();
    iteration::main();
}
