use rust_stark::core::algebra::{xgcd, Field, FieldElement};
use rust_stark::core::univariate::Polynomial;

fn test_distributivity() {
    let field = Field::main();
    let zero = field.zero();
    let one = field.one();
    let two = FieldElement::new(2, field);
    let five = FieldElement::new(5, field);

    let a = Polynomial::new(vec![one, zero, five, two]);
    let b = Polynomial::new(vec![two, two, one]);
    let c = Polynomial::new(vec![zero, five, two, five, five, one]);

    let lhs = a.clone() * (b.clone() + c.clone());
    let rhs = (a.clone() * b.clone()) + (a.clone() * c.clone());

    assert_eq!(lhs, rhs, "Error! distributivity fails for polynomials!");

    println!("univariate polynomial distributivity success!");
}

fn main() {
    let f: Field = Field::new(7);
    let fe1: FieldElement = FieldElement::new(4, f);
    let fe2: FieldElement = FieldElement::new(5, f);
    println!("{:?}", fe1 + fe2);

    let (x, y) = (98 as i128, 56 as i128);
    let out: (i128, i128, i128) = xgcd(x, y);
    println!("{:?}", out);
    println!("{0} {1}", x, y);

    test_distributivity();
}
