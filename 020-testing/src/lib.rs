pub use shape::Circle;

mod shape {
    pub struct Circle {
        radius: f32,
    }

    impl Circle {
        pub fn new(radius: f32) -> Circle {
            Circle { radius }
        }

        pub fn new_with_result(radius: f32) -> Result<Circle, String> {
            if radius <= 0.0 {
                return Err(String::from("radius must be greater than 0"));
            }

            return Ok(Circle { radius });
        }

        /// # Example
        /// ```
        /// use testing::Circle;
        /// let c1 = Circle::new(10.0);
        /// let c2 = Circle::new(5.0);
        /// assert!(c1.contains(&c2));
        /// ```
        pub fn contains(&self, other: &Circle) -> bool {
            self.radius > other.radius
        }
    }
}

#[cfg(test)] // use #[cfg(test)] to only compile the tests when running tests
mod tests {
    use super::*; // we should import the module we want to test, super means the parent module and * means import everything

    #[test] // we should use the #[test] attribute to define a test function
    fn test_larger_contains_smaller() {
        let larger_circle = shape::Circle::new(10.0);
        let smaller_cirlcle = shape::Circle::new(5.0);

        assert!(
            larger_circle.contains(&smaller_cirlcle),
            "This is a custom error message"
        );
    }

    #[test]
    fn test_smaller_should_not_contain_larger() {
        let smaller = shape::Circle::new(2.1);
        let larger = shape::Circle::new(4.3);
        assert_eq!(smaller.contains(&larger), false);
    }

    #[test]
    fn test_should_not_create_circle_with_negative_radius() {
        let c = Circle::new_with_result(-1.9);
        assert!(
            c.is_err(),
            "Circle with negative radius should return an error"
        );
    }

    #[test]
    fn test_can_create_circle_with_positive_radius() {
        let c = Circle::new_with_result(1.2);
        assert!(c.is_ok(), "Circle with positive radius should be created");
    }

    #[test]
    fn test_should_not_create_circle_with_negative_radius_using_question_mark() -> Result<(), String>
    {
        // this question mark will return the error if the result is an error. otherwise it will return the value of OK.
        // in this way, rust will automatically do the assertion for us.
        let c = Circle::new_with_result(-1.9)?;
        Ok(())
    }
}
