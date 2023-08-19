struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("x-eddie"),
        email: String::from("eddie@hey.com"),
        sign_in_count: 37,
    };

    let mut user2 = User {
        active: true,
        username: user1.username,
        email: user1.email,
        sign_in_count: 3,
    };

    user2.username = String::from("reified");

    let user3 = build_user(String::from("nisal"), String::from("nisal@hey.com"));

    let _user4 = User {
        username: String::from("other"),
        ..user3
    };
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
