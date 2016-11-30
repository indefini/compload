#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

#[no_mangle]
pub fn get_my_i32() -> i32
{
    println!("sssso");
    44i32
}
