
#[derive(Debug)]
struct NoneCopy;

#[test]
fn slice_iter() {
    let arr = [NoneCopy, NoneCopy];

    for item in arr.into_iter() {
        println!("{:?}", item);
    }


    // println!("{arr:?}");

}