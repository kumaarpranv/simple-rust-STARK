use std::cmp::PartialEq;
use std::ops;

pub fn xgcd(mut x: i128, mut y: i128) -> (i128, i128, i128) {
    let (mut a0, mut a1, mut b0, mut b1) = (1, 0, 0, 1);
    while y != 0 {
        // dbg!(x,y);
        let (q, r) = (x / y, x % y);
        let (c, d) = (a0 - q * a1, b0 - q * b1);

        x = y;
        y = r;
        a0 = a1;
        a1 = c;
        b0 = b1;
        b1 = d;
    }
    (x, a0, b0)
}

#[derive(Clone, Copy, Debug)]
pub struct FieldElement {
    value: i128,
    pub field: Field,
}

impl FieldElement {
    pub fn new(value: i128, field: Field) -> FieldElement {
        FieldElement {
            value: value,
            field: field,
        }
    }

    pub fn inverse(self) -> FieldElement {
        return self.field.inverse(self);
    }

    pub fn is_zero(self) -> bool {
        return self.value == 0;
    }
}

impl ops::Add<FieldElement> for FieldElement {
    type Output = FieldElement;
    fn add(self, _rhs: FieldElement) -> FieldElement {
        return self.field.add(self, _rhs);
    }
}

impl ops::Sub<FieldElement> for FieldElement {
    type Output = FieldElement;
    fn sub(self, _rhs: FieldElement) -> FieldElement {
        return self.field.subtract(self, _rhs);
    }
}

impl ops::Mul<FieldElement> for FieldElement {
    type Output = FieldElement;
    fn mul(self, _rhs: FieldElement) -> FieldElement {
        return self.field.multiply(self, _rhs);
    }
}

impl ops::Div<FieldElement> for FieldElement {
    type Output = FieldElement;
    fn div(self, _rhs: FieldElement) -> FieldElement {
        return self.field.divide(self, _rhs);
    }
}

impl ops::Neg for FieldElement {
    type Output = FieldElement;
    fn neg(self) -> FieldElement {
        return self.field.negate(self);
    }
}

impl ops::BitXor<i128> for FieldElement {
    type Output = FieldElement;
    fn bitxor(self, exponent: i128) -> FieldElement {
        let mut acc = FieldElement::new(1, self.field);
        let val = FieldElement::new(self.value, self.field);
        for i in (0..format!("{:b}", exponent).chars().count()).rev() {
            acc = acc * acc;
            if (1 << i) & exponent != 0 {
                acc = acc * val;
            }
        }

        return acc;
    }
}

impl PartialEq<FieldElement> for FieldElement {
    fn eq(&self, other: &FieldElement) -> bool {
        return (*self).value == (*other).value;
    }

    fn ne(&self, other: &FieldElement) -> bool {
        return (*self).value != (*other).value;
    }
}

#[derive(Clone, Debug, Copy)]
pub struct Field {
    p: i128,
}

impl Field {
    pub fn new(p: i128) -> Field {
        Field { p: p }
    }
    pub fn zero(self) -> FieldElement {
        FieldElement::new(0, self)
    }

    pub fn one(self) -> FieldElement {
        FieldElement::new(1, self)
    }

    pub fn add(self, left: FieldElement, right: FieldElement) -> FieldElement {
        return FieldElement::new(((left).value + (right).value) % self.p, self);
    }

    pub fn subtract(self, left: FieldElement, right: FieldElement) -> FieldElement {
        return FieldElement::new((self.p + (left).value + (right).value) % self.p, self);
    }

    pub fn multiply(self, left: FieldElement, right: FieldElement) -> FieldElement {
        return FieldElement::new(((left).value * (right).value) % self.p, self);
    }

    pub fn negate(self, operand: FieldElement) -> FieldElement {
        return FieldElement::new((self.p - (operand).value) % self.p, self);
    }

    pub fn inverse(self, operand: FieldElement) -> FieldElement {
        let (a, _b, _g) = xgcd((operand).value as i128, self.p);
        return FieldElement::new(a, self);
    }

    pub fn divide(self, left: FieldElement, right: FieldElement) -> FieldElement {
        assert_ne!((right).value, 0, "divide by zero");
        let (a, _b, _g) = xgcd((right).value as i128, self.p);
        return FieldElement::new((left).value * a % self.p, self);
    }

    pub fn main() -> Field {
        let base: i128 = 2;
        let p: i128 = 19; //1 + 407 * base.pow(119);
        return Field::new(p);
    }

    pub fn generator(self) -> Option<FieldElement> {
        let base: i128 = 2;

        assert_eq!(
            self.p,
            1 + 407 * base.pow(119),
            "Do not know generator for other fields beyond 1+407*2^119"
        );
        return Some(FieldElement::new(
            85408008396924667383611388730472331217,
            self,
        ));
    }

    pub fn primitive_nth_root(self, n: i128) -> Option<FieldElement> {
        let base: i128 = 2;
        if self.p == 1 + 407 * base.pow(119) {
            assert!(
                n <= base.pow(119),
                "Field does not have nth root of unity where n > 2^119 or not power of two."
            );
            let mut root: FieldElement =
                FieldElement::new(85408008396924667383611388730472331217, self);
            let mut order: i128 = base.pow(119);
            while order != n {
                root = root ^ 2;
                order = order / 2;
            }
            return Some(root);
        } else {
            assert!(false, "Unknown field, can't return root of unity.");
            return None;
        }
    }

    pub fn sample(self, byte_array: Vec<u8>) -> FieldElement {
        let mut acc: i128 = 0;
        for b in byte_array {
            acc = (acc << 8) ^ (b as i128);
        }
        return FieldElement::new(acc % self.p, self);
    }
}
