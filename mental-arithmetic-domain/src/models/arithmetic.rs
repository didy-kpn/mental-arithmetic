use super::constant_value::ConstantValue;
use super::operator::{Operator, Operators};

use rand::{
    distributions::{Distribution, Uniform},
    seq::IteratorRandom,
};

#[derive(Copy, Clone, Debug)]
pub struct Arithmetic {
    operator: Operator,
    left_value: ConstantValue,
    right_value: ConstantValue,
    result: ConstantValue,
}

impl Arithmetic {
    pub fn new(operators: Operators) -> Self {
        let operator = operators.get_by_random();
        match operator {
            Operator::Addition => Self::make_addition(),
            Operator::Subtraction => Self::make_subtraction(),
            Operator::Multiplication => Self::make_multiplication(),
            Operator::Division => Self::make_division(),
        }
    }

    pub fn try_answer(&self, value: ConstantValue) -> bool {
        self.result == value
    }

    pub fn get_express(&self) -> (ConstantValue, Operator, ConstantValue) {
        (self.left_value, self.operator, self.right_value)
    }

    pub fn get_result(&self) -> ConstantValue {
        self.result
    }

    fn make_addition() -> Self {
        let (left_value, right_value) = {
            let mut rng = rand::thread_rng();
            let l = Uniform::from(1..=50).sample(&mut rng);
            let r = Uniform::from(1..=l).sample(&mut rng);
            (l.into(), r.into())
        };
        let result = left_value + right_value;

        Self {
            operator: Operator::Addition,
            left_value,
            right_value,
            result,
        }
    }

    fn make_subtraction() -> Self {
        let (left_value, right_value) = {
            let mut rng = rand::thread_rng();
            let l = Uniform::from(1..=50).sample(&mut rng);
            let r = Uniform::from(1..=l).sample(&mut rng);
            (l.into(), r.into())
        };
        let result = left_value - right_value;

        Self {
            operator: Operator::Subtraction,
            left_value,
            right_value,
            result,
        }
    }

    fn make_multiplication() -> Self {
        let (left_value, right_value) = {
            let mut rng = rand::thread_rng();
            let l = Uniform::from(1..=10).sample(&mut rng);
            let r = Uniform::from(1..=l).sample(&mut rng);
            (l.into(), r.into())
        };
        let result = left_value * right_value;

        Self {
            operator: Operator::Multiplication,
            left_value,
            right_value,
            result,
        }
    }

    fn make_division() -> Self {
        let (left_value, right_value) = {
            let mut rng = rand::thread_rng();
            let l = Uniform::from(1..=100).sample(&mut rng);
            let r = (1..=l).filter(|&n| l % n == 0).choose(&mut rng).unwrap();

            (l.into(), r.into())
        };
        let result = left_value / right_value;

        Self {
            operator: Operator::Division,
            left_value,
            right_value,
            result,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::operator::{
        AdditionEnabled, DivisionEnabled, MultiplicationEnabled, Operators, SubtractionEnabled,
    };
    use super::Arithmetic;

    #[test]
    fn test_arithmetic_correct() {
        let arithmetic = Arithmetic::new(Operators::new(
            AdditionEnabled::from(false),
            SubtractionEnabled::from(true),
            MultiplicationEnabled::from(false),
            DivisionEnabled::from(true),
        ));

        let result = arithmetic.get_result();

        assert!(arithmetic.try_answer(result));
    }

    #[test]
    fn test_arithmetic_incorrect() {
        let arithmetic = Arithmetic::new(Operators::new(
            AdditionEnabled::from(false),
            SubtractionEnabled::from(true),
            MultiplicationEnabled::from(false),
            DivisionEnabled::from(true),
        ));

        let result = arithmetic.get_result() + 1.into();

        assert!(arithmetic.try_answer(result) == false);
    }
}
