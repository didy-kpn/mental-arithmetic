use crate::queries::get_arithmetic_express::GetArithmeticExpress;

pub trait ArithmeticExpressPresenter<V> {
    fn output(&self, express: GetArithmeticExpress) -> V;
}
