pub trait Vars<Item> {
    
}

pub trait Tuple {

}

pub trait UnCons<Item, Items: Tuple> {
    fn uncons(&self) -> (Item, Items); 
}

// pub trait PushFront<Item> {
//     fn push_front(&self, item: Item) -> impl Tuple;
// }

// impl <Item, Target> Vars<Item> for Target
//     where Target: Clone {

// } 

