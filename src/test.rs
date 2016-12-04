use std::rc::Rc;
use std::cell::RefCell;
use dormin::component::{Component,Components, CompTrait};
use dormin::component::manager::Encode;
use dormin::object::Object;
use dormin::transform;
use rustc_serialize::{json, Encodable, Encoder, Decoder, Decodable};
use dormin::resource;

use dormin::property::{PropertyRead, PropertyGet, PropertyWrite, WriteValue};
use std::any::Any;
use dormin::input;
use dormin::component;


#[derive(Clone, RustcEncodable, RustcDecodable,Default)]
pub struct Test
{
    pub speed : f64
}

#[derive(Clone)]
pub struct TestBehavior;

impl Test
{
    pub fn nefw() -> Test
    {
        Test {
            speed : 3f64
        }
    }
}

/*
pub fn test_new(ob : &Object, resource : &resource::ResourceGroup) -> Box<Components>
{
    box Components::TestBehavior(TestBehavior)
}
*/

impl Component for TestBehavior
{
    fn update(&mut self, ob : &mut Object, dt : f64, input : &input::Input)
    {
    }

    fn get_name(&self) -> String {
        "test_behavior".to_owned()
    }
}

property_set_impl!(Test,[speed]);
property_get_impl!(Test,[speed]);


impl CompTrait for Test
{
//    const ID  : &'static str = "test";
    /*
    fn new() -> Self
    {
        Test {
            speed : 3f64
        }
    }
    */

    fn update(&mut self) {
        println!("updating test");
    }
}


pub struct Test2
{

}

impl CompTrait for Test2
{
    fn update(&mut self) {
        println!("updating test");
    }

}

