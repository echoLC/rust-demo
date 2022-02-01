
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

    print_suit(PokerSuit::Diamonds);
    print_suit(PokerSuit::Hearts);

    let mut some = Some(5);
    let absent_num: Option<u32> = None;
    some = None;

    println!("Option None:{:?}", some);

    let six = plus_one(Some(5));
    let none = plus_one(None);

    println!("six:{:?},none:{:?}", six, none);
}

#[derive(Debug)]
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts
}

fn print_suit (card: PokerSuit) {
    println!("{:?}", card);
}

fn plus_one (x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) =>Some(i + 1)
    }
}