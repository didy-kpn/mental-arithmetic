use crate::queries::solve_arithmetic::SolveArithmetic;

pub trait SolveArithmeticPresenter<V> {
    fn output(&self, solve: SolveArithmetic) -> V;
}
