use crate::{
    commands::try_answer::TryAnswer, presenters::solve_arithmetic::SolveArithmeticPresenter,
    queries::solve_arithmetic::SolveArithmetic,
};
use mental_arithmetic_domain::repositories::arithmetic::ArithmeticRepository;

pub trait SolveArithmeticUseCase<APV> {
    fn execute(&self, answer: TryAnswer) -> APV;
}

#[derive(Clone, Copy)]
pub struct SolveArithmeticInteractor<AR, AP> {
    arithmetic_repository: AR,
    solve_arithmetic_presenter: AP,
}

impl<AR, AP> SolveArithmeticInteractor<AR, AP> {
    pub fn new(arithmetic_repository: AR, solve_arithmetic_presenter: AP) -> Self {
        Self {
            arithmetic_repository,
            solve_arithmetic_presenter,
        }
    }
}

impl<AR, AP, APV> SolveArithmeticUseCase<APV> for SolveArithmeticInteractor<AR, AP>
where
    AR: ArithmeticRepository,
    AP: SolveArithmeticPresenter<APV>,
{
    fn execute(&self, answer: TryAnswer) -> APV {
        self.solve_arithmetic_presenter.output(SolveArithmetic::new(
            self.arithmetic_repository
                .get()
                .unwrap()
                .try_answer(answer.into()),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::{SolveArithmeticInteractor, SolveArithmeticUseCase};
    use crate::{
        commands::try_answer::TryAnswer, presenters::solve_arithmetic::SolveArithmeticPresenter,
        queries::solve_arithmetic::SolveArithmetic,
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

    struct MockSolveArithmeticPresenter;
    impl MockSolveArithmeticPresenter {
        fn new() -> Self {
            Self
        }
    }
    impl SolveArithmeticPresenter<String> for MockSolveArithmeticPresenter {
        fn output(&self, result: SolveArithmetic) -> String {
            format!("{}", result.value())
        }
    }

    #[test]
    fn test_solve_arithmetic() {
        let interactor = SolveArithmeticInteractor::new(
            MockArithmeticRepository::new(None),
            MockSolveArithmeticPresenter::new(),
        );

        assert!(panic::catch_unwind(move || {
            print!("{}", interactor.execute(TryAnswer::new(0)));
        })
        .is_err());

        let interactor = SolveArithmeticInteractor::new(
            MockArithmeticRepository::new(Some(Arithmetic::new(Operators::new(
                true.into(),
                true.into(),
                true.into(),
                true.into(),
            )))),
            MockSolveArithmeticPresenter::new(),
        );

        assert!(panic::catch_unwind(move || {
            print!("{}", interactor.execute(TryAnswer::new(0)));
        })
        .is_ok());
    }
}
