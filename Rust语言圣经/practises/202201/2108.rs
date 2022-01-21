pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
pub trait Draw {
    fn draw(&self);
}
impl Draw for Button {
    fn draw(&self) {
        println!("draw Button");
    }
}
pub struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("draw SelectBox");
    }
}
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

fn main() {
    let button = Button {
        width: 10,
        height: 10,
        label: String::from("Button"),
    };
    let selectBox = SelectBox {
        width: 10,
        height: 10,
        options: vec!["1".to_string(), "2".to_string()],
    };
    let screen = Screen {
        components: vec![Box::new(button), Box::new(selectBox)],
    };

    screen.run();
}
