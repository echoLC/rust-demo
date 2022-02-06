
pub trait Draw {
    fn draw(&self);
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("绘制 button");
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("绘制 selectBox");
    }
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run (&self) {
        for component in self.components.iter() {
            component.draw()
        }
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(
                SelectBox {
                    width: 20,
                    height: 20,
                    options: vec![
                        String::from("Yes"),
                        String::from("No"),
                        String::from("MayBe"),
                    ]
                }
            ),
            Box::new(Button{
                width: 30,
                height: 30,
                label: String::from("button")
            })
        ]
    };
    screen.run();

}


