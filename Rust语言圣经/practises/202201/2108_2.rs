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
pub struct Screen<T: Draw> {
    pub components: Vec<T>, //Draw 特征对象：Box<dyn Draw>
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw(); //FIXME
        }
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 10,
                height: 10,
                label: String::from("Button"),
            }),
            Box::new(SelectBox {
                width: 10,
                height: 10,
                options: vec!["1".to_string(), "2".to_string()],
            }),
        ],
    };

    screen.run();
}
