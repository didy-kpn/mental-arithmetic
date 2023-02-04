use mental_arithmetic_usecase::{
    presenters::solve_arithmetic::SolveArithmeticPresenter,
    queries::solve_arithmetic::SolveArithmetic,
};

#[derive(Clone, Copy)]
pub struct PrimitiveSolveArithmeticPresenter;

impl PrimitiveSolveArithmeticPresenter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PrimitiveSolveArithmeticPresenter {
    fn default() -> Self {
        Self::new()
    }
}

impl SolveArithmeticPresenter<bool> for PrimitiveSolveArithmeticPresenter {
    fn output(&self, solved: SolveArithmetic) -> bool {
        solved.value()
    }
}
