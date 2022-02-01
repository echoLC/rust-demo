
fn main() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let user1 = User {
        username: String::from("echoLC"),
        email: String::from("xx@qq.com"),
        sign_in_count: 30,
        active: false
    };

    let user2 = User {
        email: String::from("651644100@qq.com"),
        ..user1
    };

    print!("{}", user2.active);
}