
enum Direction {
    East,
    West,
    North, 
    South
}

enum Coins {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

enum IpAddr {
    Ipv4,
    Ipv6
}

enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorToRgb(u16, u16, u16)
}

struct Point {
    x: i32,
    y: i32,
    z: i32
}

fn main() {
   let direction = Direction::South;
   match direction {
       Direction::East => println!("East"),
       Direction::North | Direction::South => {
           println!("South or North");
       },
       _ => println!("west"),
   }

   let penny = Coins::Penny;
   let value = value_in_cents(penny);
   println!("penny value is: {}", value);

   let ip = IpAddr::Ipv4;
   println!("ip value is: {}", get_ip_addr(ip));

   // 模式绑定
   show_actions();

   let some_u8_value = Some(3);

   // if let
   if let Some(3) = some_u8_value {
       println!("Some(3) matched");
   }

   test_macro();

   let point = Point {x: 1, y: 2, z: 3};
   let Point{x, ..} = point;
   print!("x value is: {}\n", x);

   let num = Some(4);

   match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(y) => println!("{}", y),
    None => (),  
   }

   at_binds();
}

fn value_in_cents (coins: Coins) -> u8 {
    match coins {
        Coins::Penny => {
            println!("Lucky penny!");
            1
        },
        Coins::Nickel => 5,
        Coins::Dime => 10,
        Coins::Quarter => 25,
    }
}

fn get_ip_addr (ip: IpAddr) -> String {
    match ip {
        IpAddr::Ipv4 => "127.0.0.1".to_string(),
        _ => "::1".to_string(),
    }
}

// 模式绑定
fn show_actions () {
    let actions = [
        Action::Say("hello".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColorToRgb(0, 0, 255)
    ];
    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            },
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            },
            Action::ChangeColorToRgb(r, g, _) => {
                println!("change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                    r, g,
                );
            }
        }
    }
}

// match! 宏
fn test_macro () {
    let age = Some(30);
    println!("在匹配前，age是{:?}",age);
    match  age {
        Some(age) => println!("匹配出来的age是{}",age),
        _ => ()
    }
    println!("在匹配后，age是{:?}",age);
}

// @ 绑定
fn at_binds () {
    enum Message {
        Hello { id: i32 }
    }
    let msg = Message::Hello{id: 5};
    match msg {
        Message::Hello {id: id_var @ 3..= 7} => {
            println!("Found an id in range: {}", id_var);
        },
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        },
        Message::Hello { id} => {
            println!("Found some other id: {}", id);
        }
    }
}
