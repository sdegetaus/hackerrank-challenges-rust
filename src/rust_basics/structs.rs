#![allow(unused_variables)]

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(u8, u8, u8); // no need to specify field names

pub fn main() {
    let user1 = User {
        email: String::from("email"),
        username: String::from("username"),
        active: true,
        sign_in_count: 3,
    };

    // we can use a spread object operator
    let user2 = User { ..user1 };
    let color_white = Color(255, 255, 255);

    let user3 = build_user(user2.email, user2.username);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
