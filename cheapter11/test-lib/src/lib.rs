#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}
impl Rectangle {
    pub fn can_hold(self: &Rectangle, rect2: &Rectangle) -> bool {
        self.width > rect2.width && self.length > rect2.length
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn can_hold() {
        let larger = Rectangle {
            width: 20,
            length: 10,
        };
        let small = Rectangle {
            width: 10,
            length: 5,
        };
        assert!(larger.can_hold(&small));
    }

    #[test]
    #[should_panic]
    fn antother() {
        panic!("Make this test failed");
    }

    #[test]
    fn it_works2() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
