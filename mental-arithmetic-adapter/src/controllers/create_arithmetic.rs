use mental_arithmetic_usecase::{
    commands::create_arithmetic::CreateArithmetic,
    interactors::create_arithmetic::CreateArithmeticUseCase,
};
use std::marker::PhantomData;

#[derive(Clone, Copy)]
pub struct CreateArthmeticController<U, V> {
    create_arthmetic_usecase: U,
    _marker: PhantomData<V>,
}

impl<U, V> CreateArthmeticController<U, V>
where
    U: CreateArithmeticUseCase<V>,
{
    pub fn new(create_arthmetic_usecase: U) -> Self {
        Self {
            create_arthmetic_usecase,
            _marker: PhantomData,
        }
    }

    pub fn execute(
        &self,
        addition: bool,
        subtraction: bool,
        multiplication: bool,
        division: bool,
    ) -> V {
        self.create_arthmetic_usecase.execute(CreateArithmetic::new(
            addition,
            subtraction,
            multiplication,
            division,
        ))
    }
}
