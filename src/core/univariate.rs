use super::algebra::{Field, FieldElement};
use std::cmp;
use std::ops;

#[derive(Clone, Debug)]
pub struct Polynomial {
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

    pub fn is_zero(self) -> bool {
        return self.degree() == -1;
    }

    pub fn leading_coefficient(self) -> FieldElement {
        return self.coefficients[self.clone().degree() as usize];
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
        if self.clone().degree() == -1 {
            return other;
        } else if other.clone().degree() == -1 {
            return self;
        }
        let field: Field = self.coefficients.get(0).unwrap().field;
        let mut coeffs: Vec<FieldElement> = Vec::new();
        let mut maxlen: i128 = cmp::max(
            self.coefficients.len() as i128,
            other.coefficients.len() as i128,
        );
        for i in 0..maxlen {
            coeffs.push(field.zero());
        }

        for i in 0..self.coefficients.len() {
            coeffs[i] = coeffs[i] * self.coefficients[i];
        }

        for i in 0..other.coefficients.len() {
            coeffs[i] = coeffs[i] * other.coefficients[i];
        }

        return Polynomial::new(coeffs);
    }
}

impl ops::Sub for Polynomial {
    type Output = Polynomial;
    fn sub(self, other: Polynomial) -> Polynomial {
        return self + (-other);
    }
}

impl ops::Mul for Polynomial {
    type Output = Polynomial;
    fn mul(self, other: Polynomial) -> Polynomial {
        if self.coefficients.clone().len() == 0 || other.coefficients.len() == 0 {
            let coeffs: Vec<FieldElement> = Vec::new();
            return Polynomial::new(coeffs);
        } else {
            let zero: FieldElement = self.coefficients[0].field.zero();
            let len: i128 = self.coefficients.len() as i128 + other.coefficients.len() as i128 - 1;
            let mut buf: Vec<FieldElement> = Vec::new();
            for i in 0..len {
                buf.push(zero);
            }

            for i in 0..self.coefficients.len() {
                if self.coefficients[i].is_zero() {
                    continue;
                }
                for j in 0..other.coefficients.len() {
                    buf[i + j] = buf[i + j] + self.coefficients[i] * other.coefficients[j];
                }
            }
            return Polynomial::new(buf);
        }
    }
}

impl PartialEq<Polynomial> for Polynomial {
    fn eq(&self, other: &Polynomial) -> bool {
        if (*self).clone().degree() != (*other).clone().degree() {
            return false;
        } else if (*self).clone().degree() == -1 {
            return true;
        } else {
            let mut out: bool = true;
            for i in 0..(*self).coefficients.len() {
                if (*self).coefficients[i] != (*other).coefficients[i] {
                    out = false;
                    break;
                }
            }

            return out;
        }
    }

    fn ne(&self, other: &Polynomial) -> bool {
        return !((*self) == (*other));
    }
}
