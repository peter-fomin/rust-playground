pub struct Triangle(TriangleType);

#[derive(PartialEq)]
enum TriangleType {
    Equilateral,
    Scalene,
    Isosceles,
}

use TriangleType::{Equilateral, Scalene, Isosceles};

impl Triangle {
    pub fn build(mut sides: [u64; 3]) -> Option<Triangle> {
        sides.sort();
        if sides.iter().any(|&a| a <= 0_u64) || sides[2] > sides[0] + sides[1] {
            return None
        } else {
            match (sides[0] == sides[1], sides[1] == sides[2], sides[2] == sides[0]){
                (false, false, false) => Some(Triangle(Scalene)),
                (true, true, true) => Some(Triangle(Equilateral)),
                _ => (Some(Triangle(Isosceles))),
            }
        } 
    }

    pub fn is_equilateral(&self) -> bool {
        self.0 == Equilateral
    }

    pub fn is_scalene(&self) -> bool {
        self.0 == Scalene
    }

    pub fn is_isosceles(&self) -> bool {
        self.0 == Isosceles
    }
}
