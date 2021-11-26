use super::algebra::{Field, FieldElement};
use std::cmp;
use std::ops;

#[derive(Clone, Debug)]
struct Polynomial {
    coefficients: Vec<FieldElement>,
}

impl Polynomial {
    pub fn new(coefficients: Vec<FieldElement>) -> Polynomial {
        let mut coefs: Vec<FieldElement> = Vec::new();
        for i in coefficients {
            coefs.push(i);
        }
        Polynomial {
            coefficients: coefs,
        }
    }

    pub fn degree(self) -> i128 {
        if self.coefficients.len() == 0 {
            return -1;
        }
        let zero: FieldElement = self.coefficients[0].field.zero();

        let mut fg: bool = true;
        let mut maxindex: i128 = 0;
        let mut ct: i128 = 0;
        for i in self.coefficients {
            if i != zero {
                fg = false;
                maxindex = ct;
            }
            ct += 1;
        }

        if fg == true {
            return -1;
        }
        return maxindex;
    }
}
impl ops::Neg for Polynomial {
    type Output = Polynomial;
    fn neg(self) -> Polynomial {
        let mut vec: Vec<FieldElement> = Vec::new();
        for i in self.coefficients {
            vec.push(-i);
        }
        return Polynomial::new(vec);
    }
}

impl ops::Add for Polynomial {
    type Output = Polynomial;
    fn add(self, other: Polynomial) -> Polynomial {
        // if self.degree() == -1 {
        //     return other;
        // } else if other.degree() == -1 {
        //     return self;
        // }
        // let field: Field = self.coefficients[0].field;
        // let mut coeffs: Vec<FieldElement> = Vec::new();
        // let mut maxlen: i128 = cmp::max(
        //     self.coefficients.len() as i128,
        //     other.coefficients.len() as i128,
        // );
        // for i in 0..maxlen {
        //     coeffs.push(field.zero());
        // }

        return self;
    }
}
