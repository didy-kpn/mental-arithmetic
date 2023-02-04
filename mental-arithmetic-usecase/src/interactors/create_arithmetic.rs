use crate::{
    commands::create_arithmetic::CreateArithmetic,
    presenters::arithmetic_express::ArithmeticExpressPresenter,
    queries::get_arithmetic_express::GetArithmeticExpress,
};
use mental_arithmetic_domain::repositories::arithmetic::ArithmeticRepository;

pub trait CreateArithmeticUseCase<APV> {
    fn execute(&self, create_arithmetic: CreateArithmetic) -> APV;
}

#[derive(Clone, Copy)]
pub struct CreateArithmeticInteractor<AR, AP> {
    arithmetic_repository: AR,
    arithmetic_express_presenter: AP,
}

impl<AR, AP> CreateArithmeticInteractor<AR, AP> {
    pub fn new(arithmetic_repository: AR, arithmetic_express_presenter: AP) -> Self {
        Self {
            arithmetic_repository,
            arithmetic_express_presenter,
        }
    }
}

impl<AR, AP, APV> CreateArithmeticUseCase<APV> for CreateArithmeticInteractor<AR, AP>
where
    AR: ArithmeticRepository,
    AP: ArithmeticExpressPresenter<APV>,
{
    fn execute(&self, create_arithmetic: CreateArithmetic) -> APV {
        self.arithmetic_repository.save(create_arithmetic.into());

        self.arithmetic_express_presenter
            .output(GetArithmeticExpress::new(
                self.arithmetic_repository.get().unwrap(),
            ))
    }
}

#[cfg(test)]
mod tests {
    use super::{CreateArithmeticInteractor, CreateArithmeticUseCase};
    use crate::{
        commands::create_arithmetic::CreateArithmetic,
        presenters::arithmetic_express::ArithmeticExpressPresenter,
        queries::get_arithmetic_express::GetArithmeticExpress,
    };
    use mental_arithmetic_domain::{
        models::arithmetic::Arithmetic, repositories::arithmetic::ArithmeticRepository,
    };
    use std::cell::Cell;
    use std::panic;

    struct MockArithmeticRepository(Cell<Option<Arithmetic>>);

    impl MockArithmeticRepository {
        fn new() -> Self {
            Self(Cell::new(None))
        }
    }

    impl ArithmeticRepository for MockArithmeticRepository {
        fn save(&self, arithmetic: Arithmetic) {
            self.0.set(Some(arithmetic));
        }
        fn get(&self) -> Option<Arithmetic> {
            self.0.get()
        }
    }

    struct MockArithmeticExpressPresenter;
    impl MockArithmeticExpressPresenter {
        fn new() -> Self {
            Self
        }
    }
    impl ArithmeticExpressPresenter<String> for MockArithmeticExpressPresenter {
        fn output(&self, express: GetArithmeticExpress) -> String {
            format!(
                "{} {} {} = ?",
                express.left_value(),
                express.operator(),
                express.right_value()
            )
        }
    }

    #[test]
    fn test_create_arithmetic() {
        let interactor = CreateArithmeticInteractor::new(
            MockArithmeticRepository::new(),
            MockArithmeticExpressPresenter::new(),
        );

        assert!(panic::catch_unwind(move || {
            println!(
                "{}",
                interactor.execute(CreateArithmetic::new(true, true, true, true))
            );
        })
        .is_ok());
    }
}
