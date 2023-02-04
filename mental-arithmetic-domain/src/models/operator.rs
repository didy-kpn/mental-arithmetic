use rand::{thread_rng, Rng};
use std::convert::Into;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl From<String> for Operator {
    fn from(operator: String) -> Self {
        match &*operator {
            "+" => Self::Addition,
            "-" => Self::Subtraction,
            "*" => Self::Multiplication,
            "/" => Self::Division,
            _ => panic!(""),
        }
    }
}

impl From<Operator> for String {
    fn from(operator: Operator) -> Self {
        match operator {
            Operator::Addition => "+".to_string(),
            Operator::Subtraction => "-".to_string(),
            Operator::Multiplication => "*".to_string(),
            Operator::Division => "/".to_string(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct AdditionEnabled(bool);

#[derive(Clone, Copy)]
pub struct SubtractionEnabled(bool);

#[derive(Clone, Copy)]
pub struct MultiplicationEnabled(bool);

#[derive(Clone, Copy)]
pub struct DivisionEnabled(bool);

impl From<bool> for AdditionEnabled {
    fn from(b: bool) -> Self {
        Self(b)
    }
}

impl From<AdditionEnabled> for bool {
    fn from(enabled: AdditionEnabled) -> bool {
        enabled.0
    }
}

impl From<bool> for SubtractionEnabled {
    fn from(b: bool) -> Self {
        Self(b)
    }
}

impl From<SubtractionEnabled> for bool {
    fn from(enabled: SubtractionEnabled) -> bool {
        enabled.0
    }
}

impl From<bool> for MultiplicationEnabled {
    fn from(b: bool) -> Self {
        Self(b)
    }
}

impl From<MultiplicationEnabled> for bool {
    fn from(enabled: MultiplicationEnabled) -> bool {
        enabled.0
    }
}

impl From<bool> for DivisionEnabled {
    fn from(b: bool) -> Self {
        Self(b)
    }
}

impl From<DivisionEnabled> for bool {
    fn from(enabled: DivisionEnabled) -> bool {
        enabled.0
    }
}

pub struct Operators {
    addition: AdditionEnabled,
    subtraction: SubtractionEnabled,
    multiplication: MultiplicationEnabled,
    division: DivisionEnabled,
}

impl Operators {
    pub fn new(
        addition: AdditionEnabled,
        subtraction: SubtractionEnabled,
        multiplication: MultiplicationEnabled,
        division: DivisionEnabled,
    ) -> Self {
        if !(addition.into() || subtraction.into() || multiplication.into() || division.into()) {
            panic!("Element of Operator required one or more");
        }

        Self {
            addition,
            subtraction,
            multiplication,
            division,
        }
    }

    pub fn get_by_random(&self) -> Operator {
        let mut v = Vec::new();
        if self.addition.into() {
            v.push(Operator::Addition);
        }

        if self.subtraction.into() {
            v.push(Operator::Subtraction);
        }

        if self.multiplication.into() {
            v.push(Operator::Multiplication);
        }

        if self.division.into() {
            v.push(Operator::Division);
        }

        let index = thread_rng().gen_range(0..v.len());
        v.into_iter().nth(index).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AdditionEnabled, DivisionEnabled, MultiplicationEnabled, Operators, SubtractionEnabled,
    };
    use std::panic;

    #[test]
    fn test_operators_new_empty() {
        let result = panic::catch_unwind(|| {
            Operators::new(
                AdditionEnabled::from(false),
                SubtractionEnabled::from(false),
                MultiplicationEnabled::from(false),
                DivisionEnabled::from(false),
            )
        });
        assert!(result.is_err());
    }
}
