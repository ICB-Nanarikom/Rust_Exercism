#[derive(Debug, PartialEq, Eq)]
pub struct Clock (i32);
const DAY_TO_SECOND: i32 = 1440;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock(60 * hours + minutes).normalize()
    }

    pub fn normalize(&self) -> Self {
        Clock((self.0 % DAY_TO_SECOND + DAY_TO_SECOND) % DAY_TO_SECOND)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock(self.0 + minutes).normalize()
    }

    pub fn to_string(&self) -> String {
        let h = self.0 / 60;
        let m = self.0 % 60;
        String::from(format!("{:02}:{:02}", h, m))
    }
}