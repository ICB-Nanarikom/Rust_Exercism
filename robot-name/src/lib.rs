use std::collections::HashSet;
use std::cell::RefCell;

thread_local!(static S: RefCell<HashSet<String>> = RefCell::new(HashSet::new()));

fn roll() -> String {
    use rand::{thread_rng, distributions::Uniform, prelude::Distribution};
    let it1 = Uniform::from('A'..='Z').sample_iter(thread_rng()).take(2);
    let it2 = Uniform::from('0'..='9').sample_iter(thread_rng()).take(3);
    it1.chain(it2).collect::<String>()
}
fn roll_while() -> String {
    S.with(|c| {
        let mut ret = roll();
        let mut s = c.borrow_mut();
        while s.contains(&ret) {
            ret = roll();
        }
        s.insert(ret.clone());
        ret
    })
}

pub struct Robot(String);

impl Robot {
    pub fn new() -> Self {
        Robot(roll_while())
    }

    pub fn name(&self) -> &str {
        self.0.as_str()
    }

    pub fn reset_name(&mut self) {
        self.0 = roll_while()
    }
}
