#[cfg(test)]
mod test;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Out of range, the number should be between 1 and 100");
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}