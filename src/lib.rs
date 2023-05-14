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
}
