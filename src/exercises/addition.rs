use rand::RngExt;

use crate::models::Number;

fn generate(min: i64, max: i64, rng: &mut rand::rngs::ThreadRng) -> Number {
    let sum = rng.random_range(min..=max);
    let a_val = rng.random_range(0..=sum);
    let b_val = sum - a_val;

    let a = Number::new_literal(a_val as isize);
    let b = Number::new_literal(b_val as isize);

    Number::new_expression(a, b, crate::models::Operation::Addition)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        for _ in 0..20 {
            // println!("{i}");
            let min = 30;
            let max = 60;

            let mut rng = rand::rng();
            let num = crate::exercises::addition::generate(min, max, &mut rng);
            assert!(&60 >= num.result());
            assert!(num.result() >= &30);
            // println!("{}", num);
        }
    }
}
