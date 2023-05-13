pub fn add(left: usize, right: usize) -> usize {
    left + right
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        if width <= 0 || height <= 0 {
            panic!("No dimension must be 0 or lower");
        }
        Rectangle { width, height }
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_can_hold(){
        let larger = Rectangle::new(8,12);
        let smaller = Rectangle::new(7,12);
        assert!(larger.can_hold(&smaller));
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    #[should_panic(expected="No dimension must be 0 or lower")]
    fn test_rectangle_new(){
        Rectangle::new(0,1);
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 4, 6);
    }
}
