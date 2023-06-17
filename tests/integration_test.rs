use vars::*;

#[test]
fn assert_true() {

    fn consume(v: impl Vars<usize>) {
        for i in v.into_iter() {
            println!("{i:?}");
        }
    }

    consume((1,2,3));
}