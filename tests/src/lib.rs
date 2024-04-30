#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool
    {
        self.width > other.width && self.height > other.height
    }
}

pub fn increment(value: i32) -> i32
{
    value + 1
}

pub fn greeting(name: &str) -> String
{
    format!("Hello {}", name)
}

pub struct Month {
    day: i32,
}

impl Month {
    pub fn set_day(day: i32) -> Month
    {
        if day < 1 || day > 31 {
            panic!("Day must be between 1 and 31. Got {}.", day);
        }
        Month{day}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller()
    {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 4,
            height: 5,
        };
        assert!(larger.can_hold(&smaller));
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn test_increment()
    {
        assert_eq!(4, increment(3));
        assert_eq!(0, increment(-1));
    }

    #[test]
    fn test_greeting()
    {
        let result = greeting("John");
        assert!(result.contains("John"));
    }

    #[test]
    #[should_panic]
    fn test_set_day()
    {
        Month::set_day(35);
    }
}



