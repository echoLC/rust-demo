struct Circle {
    x: f64,
    y: f64,
    radius: f64
}

struct Rectangle {
    width: u32,
    height: u32
}

impl Circle {
    fn new (x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x,
            y,
            radius
        }
    }

    fn area (&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

impl Rectangle {
    fn area (&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
   let circle = Circle { x: 0.0, y: 0.0, radius: 20.0};
   let area = circle.area();
   println!("The area of circle which radius is 20 is: {}", area);

   let rect = Rectangle{ width: 30, height: 40};
   println!("The area of the rectangle is {} square pixels.", rect.area());
   if rect.width() {
        println!("The with of the rectangle is positive.");
   }
}


