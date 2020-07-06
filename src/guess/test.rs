use super::*;

#[test]
#[should_panic]
fn greater_than_100() {
    Guess::new(200);
}
