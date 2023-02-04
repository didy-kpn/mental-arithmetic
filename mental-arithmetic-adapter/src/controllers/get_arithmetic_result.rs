use mental_arithmetic_usecase::interactors::get_arithmetic_result::GetArithmeticResultUseCase;
use std::marker::PhantomData;

#[derive(Clone, Copy)]
pub struct GetArthmeticResultController<U, V> {
    get_arithmetic_result_usecase: U,
    _marker: PhantomData<V>,
}

impl<U, V> GetArthmeticResultController<U, V>
where
    U: GetArithmeticResultUseCase<V>,
{
    pub fn new(get_arithmetic_result_usecase: U) -> Self {
        Self {
            get_arithmetic_result_usecase,
            _marker: PhantomData,
        }
    }

    pub fn execute(&self) -> V {
        self.get_arithmetic_result_usecase.execute()
    }
}
