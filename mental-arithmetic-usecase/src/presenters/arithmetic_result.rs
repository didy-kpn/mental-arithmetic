use crate::queries::get_arithmetic_result::GetArithmeticResult;

pub trait ArithmeticResultPresenter<V> {
    fn output(&self, result: GetArithmeticResult) -> V;
}
