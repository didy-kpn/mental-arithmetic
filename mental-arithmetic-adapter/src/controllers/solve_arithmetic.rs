use mental_arithmetic_usecase::{
    commands::try_answer::TryAnswer, interactors::solve_arithmetic::SolveArithmeticUseCase,
};
use std::marker::PhantomData;

#[derive(Clone, Copy)]
pub struct SolveArithmeticController<U, V> {
    solve_arithmetic_usecase: U,
    _marker: PhantomData<V>,
}

impl<U, V> SolveArithmeticController<U, V>
where
    U: SolveArithmeticUseCase<V>,
{
    pub fn new(solve_arithmetic_usecase: U) -> Self {
        Self {
            solve_arithmetic_usecase,
            _marker: PhantomData,
        }
    }

    pub fn execute(&self, answer: i32) -> V {
        self.solve_arithmetic_usecase
            .execute(TryAnswer::new(answer))
    }
}
