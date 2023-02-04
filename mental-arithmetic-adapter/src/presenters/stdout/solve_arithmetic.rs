use mental_arithmetic_usecase::{
    presenters::solve_arithmetic::SolveArithmeticPresenter,
    queries::solve_arithmetic::SolveArithmetic,
};

#[derive(Clone, Copy)]
pub struct StdoutSolveArithmeticPresenter;

impl StdoutSolveArithmeticPresenter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for StdoutSolveArithmeticPresenter {
    fn default() -> Self {
        Self::new()
    }
}

impl SolveArithmeticPresenter<()> for StdoutSolveArithmeticPresenter {
    fn output(&self, solved: SolveArithmetic) {
        println!(
            "answer is {}!",
            if solved.value() {
                "correct"
            } else {
                "incorrect"
            }
        )
    }
}
