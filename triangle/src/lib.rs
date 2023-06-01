pub struct Triangle([u64; 3]);

impl Triangle {
    pub fn build(mut sides: [u64; 3]) -> Option<Triangle> {
        sides.sort();
        if sides[0] + sides[1] <= sides[2] { None } else { Some(Triangle(sides)) }
    }

    pub fn is_equilateral(&self) -> bool {
        self.0[0] == self.0[1] && self.0[1] == self.0[2]
    }

    pub fn is_scalene(&self) -> bool {
        self.is_equilateral() == false && self.is_isosceles() == false
    }

    pub fn is_isosceles(&self) -> bool {
        self.0[0] == self.0[1] || self.0[1] == self.0[2]
    }
}
