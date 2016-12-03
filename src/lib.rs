#![feature(associated_consts)]

#[macro_use]
extern crate dormin;
extern crate rustc_serialize;

use std::collections::HashMap;
use dormin::component::CompTrait;

mod test;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

#[no_mangle]
pub fn get_my_i32() -> i32
{
    println!("dsfsfsfdsfs");
    11113i32
}

pub fn get_comp_list() -> Vec<String>
{
    vec!["test".to_owned(), "dance".to_owned()]
}

pub struct Components
{
    t : Vec<test::Test>,
    t2 : Vec<test::Test>
    //... etc
}

//pub type ObjectComponents2 = HashMap<String, Vec<u32>>;
struct World {
    objects : Vec<HashMap<String, Vec<u32>>>,
    vacant : Vec<u32>,
    components : Components
}

impl World
{
    fn update(&self) {

    }

    fn get<T : CompTrait>(&self, id : usize) -> Option<T> {
        let map = &self.objects[id];
        if let Some(ref m) = map.get(T::ID) {

            /*
            match T::ID {
                "test" => Some(self.components.t[0] as T),
                _ => None
            }
            */
            None

        }
        else {
            None
        }
    }
}

/*
impl CompStore
{
    //fn get<T : CompTrait>(&self, index : u32) -> Option<T>
    fn get<T>(&self, index : u32) -> Option<T>
    {
        None
    }

}
*/
