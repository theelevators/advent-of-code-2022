use core::slice;
use std::collections::HashSet;

pub type Frequency = i32;
pub struct Handheld {
    frequency: Frequency,
}

impl Handheld {
    pub fn new() -> Handheld {
        Handheld {
            frequency: 0,
        }
    }
    pub fn set_frequency(&mut self, frequency: Frequency) {
        self.frequency = frequency;
    }

    pub fn read_signals(&self, buffer: &str) {
        let mut start = 0;
        let mut end = self.frequency as usize;

        while end <= buffer.len() {
            let slice = &buffer[start..end];
            let unique: HashSet<_> = slice.chars().collect();
            match slice.len() == unique.len() {
                true => {
                    println!("First marker after character: {end}");
                    break;
                }
                false => {
                    start += 1;
                    end += 1;
                }
            }
        }
    }
}
