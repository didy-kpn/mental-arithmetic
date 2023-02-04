use mental_arithmetic_usecase::{
    presenters::arithmetic_express::ArithmeticExpressPresenter,
    queries::get_arithmetic_express::GetArithmeticExpress,
};

#[derive(Clone, Copy)]
pub struct PrimitiveArithmeticExpressPresenter;

impl PrimitiveArithmeticExpressPresenter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PrimitiveArithmeticExpressPresenter {
    fn default() -> Self {
        Self::new()
    }
}

impl ArithmeticExpressPresenter<(i32, String, i32)> for PrimitiveArithmeticExpressPresenter {
    fn output(&self, express: GetArithmeticExpress) -> (i32, String, i32) {
        (
            express.left_value(),
            express.operator(),
            express.right_value(),
        )
    }
}
