use crate::{
    presenters::arithmetic_result::ArithmeticResultPresenter,
    queries::get_arithmetic_result::GetArithmeticResult,
};
use mental_arithmetic_domain::repositories::arithmetic::ArithmeticRepository;

pub trait GetArithmeticResultUseCase<APV> {
    fn execute(&self) -> APV;
}

#[derive(Clone, Copy)]
pub struct GetArithmeticResultInteractor<AR, AP> {
    arithmetic_repository: AR,
    arithmetic_result_presenter: AP,
}

impl<AR, AP> GetArithmeticResultInteractor<AR, AP> {
    pub fn new(arithmetic_repository: AR, arithmetic_result_presenter: AP) -> Self {
        Self {
            arithmetic_repository,
            arithmetic_result_presenter,
        }
    }
}

impl<AR, AP, APV> GetArithmeticResultUseCase<APV> for GetArithmeticResultInteractor<AR, AP>
where
    AR: ArithmeticRepository,
    AP: ArithmeticResultPresenter<APV>,
{
    fn execute(&self) -> APV {
        self.arithmetic_result_presenter
            .output(GetArithmeticResult::new(
                self.arithmetic_repository.get().unwrap(),
            ))
    }
}

#[cfg(test)]
mod tests {
    use super::{GetArithmeticResultInteractor, GetArithmeticResultUseCase};
    use crate::{
        presenters::arithmetic_result::ArithmeticResultPresenter,
        queries::get_arithmetic_result::GetArithmeticResult,
    };
    use mental_arithmetic_domain::{
        models::arithmetic::Arithmetic, models::operator::Operators,
        repositories::arithmetic::ArithmeticRepository,
    };
    use std::panic;

    struct MockArithmeticRepository(Option<Arithmetic>);

    impl MockArithmeticRepository {
        fn new(arithmetic: Option<Arithmetic>) -> Self {
            Self(arithmetic)
        }
    }

    impl ArithmeticRepository for MockArithmeticRepository {
        fn save(&self, _: Arithmetic) {
            todo!()
        }
        fn get(&self) -> Option<Arithmetic> {
            self.0
        }
    }

    struct MockArithmeticResultPresenter;
    impl MockArithmeticResultPresenter {
        fn new() -> Self {
            Self
        }
    }
    impl ArithmeticResultPresenter<String> for MockArithmeticResultPresenter {
        fn output(&self, result: GetArithmeticResult) -> String {
            format!("{}", result.value())
        }
    }

    #[test]
    fn test_get_arithmetic_result() {
        let interactor = GetArithmeticResultInteractor::new(
            MockArithmeticRepository::new(None),
            MockArithmeticResultPresenter::new(),
        );

        assert!(panic::catch_unwind(move || {
            print!("{}", interactor.execute());
        })
        .is_err());

        let interactor = GetArithmeticResultInteractor::new(
            MockArithmeticRepository::new(Some(Arithmetic::new(Operators::new(
                true.into(),
                true.into(),
                true.into(),
                true.into(),
            )))),
            MockArithmeticResultPresenter::new(),
        );

        assert!(panic::catch_unwind(move || {
            print!("{}", interactor.execute());
        })
        .is_ok());
    }
}
