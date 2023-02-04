use mental_arithmetic_usecase::{
    presenters::arithmetic_express::ArithmeticExpressPresenter,
    queries::get_arithmetic_express::GetArithmeticExpress,
};

#[derive(Clone, Copy)]
pub struct StdoutArithmeticExpressPresenter;

impl StdoutArithmeticExpressPresenter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for StdoutArithmeticExpressPresenter {
    fn default() -> Self {
        Self::new()
    }
}

impl ArithmeticExpressPresenter<()> for StdoutArithmeticExpressPresenter {
    fn output(&self, express: GetArithmeticExpress) {
        println!(
            "{} {} {} = ?",
            express.left_value(),
            express.operator(),
            express.right_value()
        )
    }
}
