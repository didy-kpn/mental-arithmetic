use mental_arithmetic_adapter::{
    controllers::{
        create_arithmetic::CreateArthmeticController,
        get_arithmetic_result::GetArthmeticResultController,
        solve_arithmetic::SolveArithmeticController,
    },
    presenters::primitive::{
        arithmetic_express::PrimitiveArithmeticExpressPresenter,
        arithmetic_result::PrimitiveArithmeticResultPresenter,
        solve_arithmetic::PrimitiveSolveArithmeticPresenter,
    },
};
use mental_arithmetic_infrastructure::inmemories::arithmetic::InMemoryArithmeticClient;
use mental_arithmetic_usecase::interactors::{
    create_arithmetic::CreateArithmeticInteractor,
    get_arithmetic_result::GetArithmeticResultInteractor,
    solve_arithmetic::SolveArithmeticInteractor,
};

#[derive(Clone)]
pub struct Arithmetic {
    create_controller: CreateArthmeticController<
        CreateArithmeticInteractor<InMemoryArithmeticClient, PrimitiveArithmeticExpressPresenter>,
        (i32, String, i32),
    >,
    get_result_controller: GetArthmeticResultController<
        GetArithmeticResultInteractor<InMemoryArithmeticClient, PrimitiveArithmeticResultPresenter>,
        i32,
    >,
    solve_controller: SolveArithmeticController<
        SolveArithmeticInteractor<InMemoryArithmeticClient, PrimitiveSolveArithmeticPresenter>,
        bool,
    >,
}

impl Arithmetic {
    pub fn create(
        &self,
        addition: bool,
        subtraction: bool,
        multiplication: bool,
        division: bool,
    ) -> (i32, String, i32) {
        self.create_controller
            .execute(addition, subtraction, multiplication, division)
    }

    pub fn solve(&self, answer: i32) -> bool {
        self.solve_controller.execute(answer)
    }

    pub fn result(&self) -> i32 {
        self.get_result_controller.execute()
    }
}

impl Default for Arithmetic {
    fn default() -> Self {
        let memory_repository = InMemoryArithmeticClient::new();

        let arithmetic_express_presenter = PrimitiveArithmeticExpressPresenter::new();
        let arithmetic_result_presenter = PrimitiveArithmeticResultPresenter::new();
        let solve_arithmetic_presenter = PrimitiveSolveArithmeticPresenter::new();

        let create_arthmetic_usecase = CreateArithmeticInteractor::new(
            memory_repository.clone(),
            arithmetic_express_presenter,
        );

        let get_arithmetic_result_usecase = GetArithmeticResultInteractor::new(
            memory_repository.clone(),
            arithmetic_result_presenter,
        );

        let solve_arithmetic_usecase =
            SolveArithmeticInteractor::new(memory_repository, solve_arithmetic_presenter);

        let create_controller = CreateArthmeticController::new(create_arthmetic_usecase);

        let get_result_controller =
            GetArthmeticResultController::new(get_arithmetic_result_usecase);

        let solve_controller = SolveArithmeticController::new(solve_arithmetic_usecase);

        Self {
            create_controller,
            get_result_controller,
            solve_controller,
        }
    }
}
