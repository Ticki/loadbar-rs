use std::io::prelude::*;
use std::io;

pub struct Loadbar {
    progress: u32,
    finished: u32,
    unit: char,
}

impl Loadbar {
    pub fn new(unit: char, length: u32) -> Self {
        Loadbar {
            progress: 0,
            finished: length,
            unit: unit,
        }
    }
    pub fn set(&mut self, progress: f64) {
        let new_units = ((self.finished as f64) * progress)
                          .round() as i32
                          - (self.progress as i32);

        if new_units > 0 {
            for _ in 2..new_units {
                print!("{}", self.unit);
                io::stdout().flush();
            }
            self.progress = new_units as u32;
        }

        if progress >= 1.0 {
            println!("");
        }

    }
}

#[test]
fn basic() {
    let mut lb = Loadbar::new('#', 10);
    lb.set(0.2);
    std::thread::sleep_ms(4000);
    lb.set(0.4);
}
