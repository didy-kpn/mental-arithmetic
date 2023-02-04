mod core;
use crate::core::CuiApplication;
use mental_arithmetic_adapter::{
    controllers::{
        create_arithmetic::CreateArthmeticController,
        get_arithmetic_result::GetArthmeticResultController,
        solve_arithmetic::SolveArithmeticController,
    },
    presenters::stdout::{
        arithmetic_express::StdoutArithmeticExpressPresenter,
        arithmetic_result::StdoutArithmeticResultPresenter,
        solve_arithmetic::StdoutSolveArithmeticPresenter,
    },
};
use mental_arithmetic_infrastructure::inmemories::arithmetic::InMemoryArithmeticClient;
use mental_arithmetic_usecase::interactors::{
    create_arithmetic::CreateArithmeticInteractor,
    get_arithmetic_result::GetArithmeticResultInteractor,
    solve_arithmetic::SolveArithmeticInteractor,
};

fn main() {
    let memory_repository = InMemoryArithmeticClient::new();

    let arithmetic_express_presenter = StdoutArithmeticExpressPresenter::new();
    let arithmetic_result_presenter = StdoutArithmeticResultPresenter::new();
    let solve_arithmetic_presenter = StdoutSolveArithmeticPresenter::new();

    let create_arthmetic_usecase =
        CreateArithmeticInteractor::new(memory_repository.clone(), arithmetic_express_presenter);

    let get_arithmetic_result_usecase =
        GetArithmeticResultInteractor::new(memory_repository.clone(), arithmetic_result_presenter);

    let solve_arithmetic_usecase =
        SolveArithmeticInteractor::new(memory_repository, solve_arithmetic_presenter);

    let create_arthmetic_controller = CreateArthmeticController::new(create_arthmetic_usecase);

    let get_arthmetic_result_controller =
        GetArthmeticResultController::new(get_arithmetic_result_usecase);

    let solve_arthmetic_controller = SolveArithmeticController::new(solve_arithmetic_usecase);

    CuiApplication::new(
        create_arthmetic_controller,
        get_arthmetic_result_controller,
        solve_arthmetic_controller,
    )
    .run();
}
