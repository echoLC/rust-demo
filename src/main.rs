
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

enum MyEnum {
    Foo,
    Bar
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
    let values = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo, MyEnum::Bar];
    let filter_values = values.iter().filter(|x| matches!(x, MyEnum::Foo));
}
