use crate::core::Vars;

impl <Item> Vars<Item> for () {}

impl <Item, Target> Vars<Item> for (Target, ) {

}