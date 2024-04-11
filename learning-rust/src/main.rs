// include a library
extern crate rand;
fn main() {
    // use rand
    let x = rand::random::<u32>();
    println!("x = {}", x);
    println!("Hello, world!");
}
