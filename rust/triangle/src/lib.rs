pub struct Triangle {
    sides: [u64; 3]
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let [a, b, c] = sides;
        if a == 0 || b == 0 || c == 0 {
            return None;
        }
        if a + b <= c || a + c <= b || b + c <= a {
            return None;
        }
        Some(Triangle{sides})
    }

    pub fn is_equilateral(&self) -> bool {
        let [a, b, c] = self.sides;
        a == b && b == c
    }

    pub fn is_scalene(&self) -> bool {   
        let [a, b, c] = self.sides;
        a != b && b != c && a != c
    }

    pub fn is_isosceles(&self) -> bool {
        let [a, b, c] = self.sides;
        !self.is_equilateral() && (a == b || b == c || a == c)
    }
}
