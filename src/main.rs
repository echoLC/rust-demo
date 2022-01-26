
fn main() {
    let s = String :: from("hello world");
    let s1 = String :: from("中国");
    let hello = &s[..5];   
    let world = &s[6..];  
    let one = &s1[..3];  
    print!("{} {}", hello, world); 
    print!("\n");
    print!("{}", one); 

    let mut s2 = String::new();
    s2.push_str("hello world");
    s2.push('!');
    assert_eq!(s2, "hello world!");

    let s3 = String::from("hello, world");
    say_hello(&s3);
    say_hello(&s3[..]);
    say_hello(&s3.as_str());

    print_str("中国人");
}

fn say_hello(str: &str) {
    print!("\n{}", str);
}

fn print_str(str: &str) {
    for c in str.chars() {
        print!("\n{}", c);
    }
}