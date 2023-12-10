pub fn add(left: usize, right: usize) -> usize {
    left + right
}

struct Rectangle {
    width: usize,
    height: usize,
}

impl Rectangle {
    fn can_fit_into(&self, other: Rectangle) -> bool {
        self.width < other.width && self.height < other.height
    }
}

struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.", value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.", value);
        }
        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::Rectangle;
    // use crate::add;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn smaller_can_fit_into_larger() {
        let smaller = Rectangle {
            width: 10,
            height: 10,
        };
        let larger = Rectangle {
            width: 20,
            height: 20,
        };
        assert!(smaller.can_fit_into(larger));
    }

    #[test]
    #[should_panic(expected = "value must be less than or equal to 100")]
    fn test_a_guess() {
        Guess::new(200);
    }
}
