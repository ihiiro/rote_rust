#[derive(Debug)]
struct Rectangle {
    _width: u32,
    _height: u32,
}

impl Rectangle {
    fn _can_hold(&self, other: &Rectangle) -> bool {
        self._width > other._width && self._height > other._height
    }
}

fn _add_five(a: i32) -> i32 {
    if a > 1000 {
        panic!("{} is larger than 1000", a);
    }
    if a < 500 {
        panic!("{} is smaller than 500", a);
    }
    a + 5
}

fn _show_name(_name: &str) -> String {
    format!("manwtf!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            _width: 8,
            _height: 7,
        };
        let smaller = Rectangle {
            _width: 5,
            _height: 1,
        };

        assert!(larger._can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            _width: 8,
            _height: 7,
        };
        let smaller = Rectangle {
            _width: 5,
            _height: 1,
        };

        assert!(!smaller._can_hold(&larger));
    }

    #[test]
    fn it_adds_five() {
        assert_eq!(10, _add_five(5));
    }

    #[test]
    fn shows_name() {
        let result = _show_name("Yassir");
        assert!(
            result.contains("Yassir"),
            "_show_name did not contain Yassir, instead it contained `{}`",
            result
        );
    }

    #[test]
    #[should_panic(expected = "is larger than 1000")]
    fn too_large() {
        _add_five(400);
    }

    #[test]
    fn fail_me() {
        panic!("This test fails!");
    }
}
