use num::complex::Complex;

fn main() {
    greet_world();
    let a: i32 = 1;
    let b: i32 = 3;
    let c = 5_i32;
    let mut d = 6i32;
    println!("The value of d is: {}", d);
    d = 7;
    let e = add(add(a, b), add(c, d));
    println!("(a + b) + (c + d) = {}", e);
    let (x, mut y): (bool, bool) = (true, false);
    println!("a = {:?}, b = {:?}", x, y);
    y = true;
    assert_eq!(x, y);
    const MAX_POSITION: u32 = 100_000;
    let spaces = "  ";
    let spaces = spaces.len();
    println!("Spaces length is {}", spaces);

    let f = Complex { re: 2.1, im: -1.2 };
    let h = Complex::new(11.1, 22.2);
    let result = f + h;

    println!("{} + {}i", result.re, result.im);
}

fn greet_world () {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "World, hello";

    let regions = [southern_germany, chinese, english];

    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn add(i: i32, j: i32) -> i32 {
    i + j
}
