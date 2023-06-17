
trait Tuple {}

impl Tuple for (usize,) {}

trait Vars<T: Tuple> {}

struct ConArr<const Len: usize = 3> {
    pub arr: [usize; Len],
}

#[test]
fn test_con_arr() {
    let array = ConArr { arr: [1,2] };
}