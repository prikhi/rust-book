fn main() {
    let mut user1 = build_user(String::from("me@you.com"), String::from("lysergia"));
    user1.email = String::from("sillyToNotHaveBareStrings");

    let _user2 = User {
        email: String::from("somethingelse@you.com"),
        ..user1
    };
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

// tuple structs - these are distinct types even though field types match
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// discrete unit types!
struct MyUnit();
