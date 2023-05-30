mod core;
mod tuple;
mod hcons;

pub use vars_macro::*;
pub use self::{core::*, hcons::*};

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

#[macro_export]
macro_rules! count_tokens {
    () => { 0 };
    ($_:tt $($tail:tt) *) => { 1 + count_tokens![$($tail) *] };
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
        assert_eq!(tup, (0,1,2,3,));
    }

    #[test]
    fn compose() {
        let sum = |a| move |b| a + b;
        let mul = |a| move |b| a * b;

        let res = (sum(1), mul(2)).compose(2);
        assert_eq!(res, 6);
    }

    #[test]
    fn variadic_function() {
        fn accept_vars(v: impl Vars<usize>) {
            let r = v.into_hcons();
        }

        let a = (1,2,3).uncons();

        accept_vars((1,2,3).push_front(0));
    }

    #[test]
    fn pop_front() {
        #[derive(Debug)]
        struct CnC(usize);
        // let t = (CnC(2),CnC(3),CnC(4),);

        // let r = t.pop_front();

        // println!("{r:?}");

        // println!("{t:?}");

        let mut tup: (usize, usize,) = (1,2,);

        let mut r = tup.uncons_mut();

        let v = (&mut (4 as usize),);

        r.1 = v;

        println!("{:?}", r);

        fn consume(t: (usize, (usize, ))) {

        }

        // consume(r.owned());

        let u_8: &usize = &10;

        let own = u_8.to_owned();
    }

    #[test]
    fn hcons_test() {
        let h = (1,2,).into_hcons();

        // let (a, b) = h.uncons();
        // let (c, d) = b.uncons();

        // let (e, f) = d.uncons();

        // match c {
        //     Some(v) => println!("{v}"),
        //     None => println!("None value found"),
        // } 
    }

}
