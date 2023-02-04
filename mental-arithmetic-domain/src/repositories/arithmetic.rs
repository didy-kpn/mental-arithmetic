use crate::models::arithmetic::Arithmetic;

pub trait ArithmeticRepository {
    fn save(&self, arithmetic: Arithmetic);
    fn get(&self) -> Option<Arithmetic>;
}
