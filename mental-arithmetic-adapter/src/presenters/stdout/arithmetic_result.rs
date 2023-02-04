use mental_arithmetic_usecase::{
    presenters::arithmetic_result::ArithmeticResultPresenter,
    queries::get_arithmetic_result::GetArithmeticResult,
};

#[derive(Clone, Copy)]
pub struct StdoutArithmeticResultPresenter;

impl StdoutArithmeticResultPresenter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for StdoutArithmeticResultPresenter {
    fn default() -> Self {
        Self::new()
    }
}

impl ArithmeticResultPresenter<()> for StdoutArithmeticResultPresenter {
    fn output(&self, result: GetArithmeticResult) {
        println!("resutl is {}", result.value())
    }
}
