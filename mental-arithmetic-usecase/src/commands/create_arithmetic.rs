use mental_arithmetic_domain::models::{
    arithmetic::Arithmetic,
    operator::{
        AdditionEnabled, DivisionEnabled, MultiplicationEnabled, Operators, SubtractionEnabled,
    },
};

#[derive(Clone)]
pub struct CreateArithmetic {
    addition: bool,
    subtraction: bool,
    multiplication: bool,
    division: bool,
}

impl CreateArithmetic {
    pub fn new(addition: bool, subtraction: bool, multiplication: bool, division: bool) -> Self {
        Self {
            addition,
            subtraction,
            multiplication,
            division,
        }
    }
}

impl From<CreateArithmetic> for Arithmetic {
    fn from(arithmetic: CreateArithmetic) -> Arithmetic {
        Arithmetic::new(Operators::new(
            AdditionEnabled::from(arithmetic.addition),
            SubtractionEnabled::from(arithmetic.subtraction),
            MultiplicationEnabled::from(arithmetic.multiplication),
            DivisionEnabled::from(arithmetic.division),
        ))
    }
}
