use mental_arithmetic_domain::models::arithmetic::Arithmetic;

pub struct GetArithmeticResult(i32);

impl GetArithmeticResult {
    pub fn new(arithmetic: Arithmetic) -> Self {
        Self(arithmetic.get_result().into())
    }
}

impl GetArithmeticResult {
    pub fn value(&self) -> i32 {
        self.0
    }
}
