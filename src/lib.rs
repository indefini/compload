#[macro_use]
extern crate dormin;
extern crate rustc_serialize;

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
