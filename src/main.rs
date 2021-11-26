use rust_stark::core::algebra::{xgcd, Field, FieldElement};

fn main() {
    let f: Field = Field::new(7);
    let fe1: FieldElement = FieldElement::new(4, &f);
    let fe2: FieldElement = FieldElement::new(5, &f);
    println!("{:?}", fe1 + fe2);

    let (x, y) = (98 as i128, 56 as i128);
    let out: (i128, i128, i128) = xgcd(x, y);
    println!("{:?}", out);
    println!("{0} {1}", x, y);
}
