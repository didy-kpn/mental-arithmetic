use mental_arithmetic_usecase::{
    presenters::arithmetic_result::ArithmeticResultPresenter,
    queries::get_arithmetic_result::GetArithmeticResult,
};

#[derive(Clone, Copy)]
pub struct PrimitiveArithmeticResultPresenter;

impl PrimitiveArithmeticResultPresenter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PrimitiveArithmeticResultPresenter {
    fn default() -> Self {
        Self::new()
    }
}

impl ArithmeticResultPresenter<i32> for PrimitiveArithmeticResultPresenter {
    fn output(&self, result: GetArithmeticResult) -> i32 {
        result.value()
    }
}
