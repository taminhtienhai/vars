mod core;
mod tuple;

pub use self::{core::*};

/// Add one number to a another given number
/// 
/// Example
/// 
/// ```
/// assert_eq!(vars::add(1,2), 3); // 3
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
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
    fn uncons() {
        let tup = (1,2,3).uncons();

        println!("{tup:?}");
    }

    #[test]
    fn push_front() {
        let tup = (1,2,3).push_front(0);
        println!("{tup:?}");
    }

    #[test]
    fn compose() {
        let sum = |a| move |b| a + b;
        let mul = |a| move |b| a * b;

        let res = (sum(1), mul(2)).compose(2);
        assert_eq!(res, 6);
    }
}
