use numx::models::Number;

fn main() {
    let x = Number::new_literal(7);
    let y = Number::new_literal(8);
    let z = Number::new_expression(x, y, numx::models::Operation::Addition);
    println!("{}", z.result());
}
