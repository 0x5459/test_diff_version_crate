
fn main() {
    dep1::append_dep1();
    mytestcrate::append("main");

    println!("{:?}", mytestcrate::get());
}

