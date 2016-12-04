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

#[no_mangle]
pub fn build_mesh()
{
    println!("will try to build mesh");
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

struct Obj {
    id : uu
}

struct Pool<T>
{
    // when you add object,
    // if data and cells
    // "data" and "cells" gets bigger.
    //
    data : Vec<T>,

    //change size for each index : can get smaller
    cells : Vec<u32>,
    free : u32,
}

impl<T:default> Pool<T>
{
    fn new() -> Pool<T>
    {
        Pool {
            data : Vec::new(),
            cells : Vec::new(),
            free : 0
        }
    }

    fn add_user(&mut self, id : u32 ) {

        if free == 0 {
            self.data.push(T::default());
            self.cells.push(id);
        }
        else if free > 0 {
            //reset?
            //self.data[len-free].reset()
            self.cells[self.cells.len()-free] = id;
            self.free = self.free -1;
        }
    }
}


// or
//rules contains object
struct TheWorldRules
{
    rule1_users : Pool<Test1>
    rule2_users : Pool<Test2>
}


struct World {
    objects : Pool<Obj>
    rules : TheWorldRules
}

impl World
{
    fn update(&self) {

    }
}

