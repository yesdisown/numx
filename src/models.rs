use std::fmt::{self};

#[derive(Clone)]
pub enum Operation {
    Addition,
    Subtraction,
    Division,
    Multiplication,
}

impl Operation {
    pub fn exec(&self, left: &Number, right: &Number) -> isize {
        match self {
            Operation::Addition => *left.result() + right.result(),
            Operation::Subtraction => *left.result() - right.result(),
            Operation::Division => *left.result() / right.result(),
            Operation::Multiplication => *left.result() * right.result(),
        }
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Operation::Addition => "+",
            Operation::Subtraction => "-",
            Operation::Division => "/",
            Operation::Multiplication => "*",
        };
        write!(f, "{}", s)
    }
}

pub enum Number {
    Literal(isize),
    Expression {
        left: Box<Number>,
        right: Box<Number>,
        op: Operation,
        result: isize,
    },
}

impl Number {
    // ===============
    // === GETTERS ===
    // ===============

    /// Returns None if the Number is a Literal variant
    /// returns the left side of the expression, if it's an Expression variant.
    pub fn left(&self) -> Option<&Number> {
        match self {
            Number::Literal(_) => None,
            Number::Expression { left, .. } => Some(left),
        }
    }

    /// Returns None if the Number is a Literal variant
    /// returns the right side of the expression, if it's an Expression variant.
    pub fn right(&self) -> Option<&Number> {
        match self {
            Number::Literal(_) => None,
            Number::Expression { left: _, right, .. } => Some(right),
        }
    }

    /// Returns None if the Number is a Literal variant
    /// returns the expression operator, if it's an Expression variant.
    pub fn op(&self) -> Option<&Operation> {
        match self {
            Number::Literal(_) => None,
            Number::Expression {
                left: _,
                right: _,
                op,
                ..
            } => Some(op),
        }
    }

    /// Returns the literal value, if the Number is a Literal variant.
    /// Returns the expression result field if it's an Expression variant.
    pub fn result(&self) -> &isize {
        match self {
            Number::Literal(n) => n,
            Number::Expression {
                left: _,
                right: _,
                op: _,
                result,
            } => result,
        }
    }

    // ====================
    // === CONSTRUCTORS ===
    // ====================

    /// constructs a new Number, of a Literal variant.
    pub fn new_literal(n: isize) -> Self {
        Number::Literal(n)
    }

    /// constructs a new Number, of an Expression variant.
    pub fn new_expression(left: Number, right: Number, op: Operation) -> Self {
        let result = op.exec(&left, &right);
        Number::Expression {
            left: Box::new(left),
            right: Box::new(right),
            op,
            result,
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Literal(n) => write!(f, "{}", n),
            Number::Expression {
                left, right, op, ..
            } => write!(f, "{} {} {}", left, op, right),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition() {
        let x = Number::new_literal(7);
        let y = Number::new_literal(8);
        let z = Number::new_expression(x, y, Operation::Addition);
        assert_eq!(z.result(), &15);
    }

    #[test]
    fn subtraction() {
        let x = Number::new_literal(7);
        let y = Number::new_literal(8);
        let z = Number::new_expression(x, y, Operation::Subtraction);
        assert_eq!(z.result(), &-1);
    }

    #[test]
    fn division() {
        let x = Number::new_literal(15);
        let y = Number::new_literal(3);
        let z = Number::new_expression(x, y, Operation::Division);
        assert_eq!(z.result(), &5);
    }

    #[test]
    fn multiplication() {
        let x = Number::new_literal(7);
        let y = Number::new_literal(8);
        let z = Number::new_expression(x, y, Operation::Multiplication);
        assert_eq!(z.result(), &56);
    }

    #[test]
    fn all_at_once() {
        let x = Number::new_literal(3);
        let y = Number::new_literal(5);
        let a = Number::new_expression(x, y, Operation::Addition); // 3 + 5 = 8

        let b = Number::new_literal(2);
        let z = Number::new_expression(a, b, Operation::Multiplication); // 8 * 2 = 16

        let c = Number::new_literal(32);
        let d = Number::new_expression(c, z, Operation::Division); // 32 / 16 = 2

        let e = Number::new_literal(3);
        let f = Number::new_expression(e, d, Operation::Subtraction); // 3 - 2 = 1
        assert_eq!(f.result(), &1);
    }
}
