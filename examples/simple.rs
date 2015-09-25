extern crate loadbar;
use loadbar::*;

fn main() {
    let mut lb = Loadbar::new('#', 10);
    lb.set(0.2);
    std::thread::sleep_ms(90);
    lb.set(0.3);
    std::thread::sleep_ms(90);
    lb.set(0.4);
    std::thread::sleep_ms(90);
    lb.set(0.5);
    std::thread::sleep_ms(90);
    lb.set(0.6);
    std::thread::sleep_ms(90);
    lb.set(0.7);
    std::thread::sleep_ms(100);
    lb.set(0.8);
    std::thread::sleep_ms(100);
    lb.set(0.9);
    std::thread::sleep_ms(100);
    lb.set(1.0);
}
