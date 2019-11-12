use pub_impl::pub_impl;

#[pub_impl]
fn test1() {
    println!("use pub_impl");
}

#[pub_impl(crate)]
fn test2() {
    println!("use pub_impl(crate)");
}

fn main() {
    test1();
    test2();
}
