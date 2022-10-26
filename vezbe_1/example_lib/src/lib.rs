//! # Example library
//! It is a collection of math operations.

// pomocu pub use izvezemo javni API za biblioteku
pub use self::operations::add;
pub use self::operations::sub;
pub use self::operations::mul;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub mod operations {
    /// Adding two numbers.
    /// # Example
    /// ```
    /// let x = 5;
    /// let y = 3;
    /// 
    /// assert_eq!(8,add(x, y));
    /// ```
    pub fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    /// Subtracting two numbers.
    /// # Example
    /// ```
    /// let x = 5;
    /// let y = 3;
    /// 
    /// assert_eq!(2,sub(x, y));
    /// ```
    pub fn sub(x: i32, y:i32) -> i32 {
        x - y
    }

    /// Multiplication two numbers.
    /// # Example
    /// ```
    /// let x = 5;
    /// let y = 3;
    /// 
    /// assert_eq!(15,mul(x, y));
    /// ```
    pub fn mul(x: i32, y:i32) -> i32 {
        x * y
    }

    /// Division two numbers.
    /// # Example
    /// ```
    /// let x = 6;
    /// let y = 3;
    /// 
    /// assert_eq!(2,div(x, y));
    /// ```
    pub fn div(x: i32, y:i32) -> i32 {
        x / y
    }
}

pub mod print {

    pub fn print_hi() {
        println!("Hello world");
    }

    pub fn print_text(text: String){
        println!("{text}");
    }
}