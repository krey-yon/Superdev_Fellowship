mod hello;
mod print;
mod shadowing;

fn main() {
    hello::hello();
    print::print();
    shadowing::transform("123");
}
