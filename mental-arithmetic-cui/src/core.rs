use mental_arithmetic_adapter::controllers::{
    create_arithmetic::CreateArthmeticController,
    get_arithmetic_result::GetArthmeticResultController,
    solve_arithmetic::SolveArithmeticController,
};
use mental_arithmetic_usecase::interactors::{
    create_arithmetic::CreateArithmeticUseCase, get_arithmetic_result::GetArithmeticResultUseCase,
    solve_arithmetic::SolveArithmeticUseCase,
};
use std::io::stdin;

pub struct CuiApplication<CU, CV, GU, GV, SU, SV> {
    create_arthmetic_controller: CreateArthmeticController<CU, CV>,
    get_arthmetic_result_controller: GetArthmeticResultController<GU, GV>,
    solve_arthmetic_controller: SolveArithmeticController<SU, SV>,
}

impl<CU, CV, GU, GV, SU, SV> CuiApplication<CU, CV, GU, GV, SU, SV>
where
    CU: CreateArithmeticUseCase<CV>,
    GU: GetArithmeticResultUseCase<GV>,
    SU: SolveArithmeticUseCase<SV>,
{
    pub fn new(
        create_arthmetic_controller: CreateArthmeticController<CU, CV>,
        get_arthmetic_result_controller: GetArthmeticResultController<GU, GV>,
        solve_arthmetic_controller: SolveArithmeticController<SU, SV>,
    ) -> Self {
        Self {
            create_arthmetic_controller,
            get_arthmetic_result_controller,
            solve_arthmetic_controller,
        }
    }

    pub fn run(&self) {
        let (addition, subtraction, multiplication, division) = select_operators();

        loop {
            self.create_arthmetic_controller.execute(
                addition,
                subtraction,
                multiplication,
                division,
            );

            let input = get_input();

            if input == "q" {
                break;
            }

            self.solve_arthmetic_controller
                .execute(input.parse::<i32>().unwrap_or(-1));

            self.get_arthmetic_result_controller.execute();
            println!();
        }
    }
}

fn get_input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Input Error");
    input.trim().to_string().to_lowercase()
}

fn select_operators() -> (bool, bool, bool, bool) {
    loop {
        let addition = loop {
            println!("Enable Addition? (Y/n)");
            let input = get_input();
            if input == "no" || input == "n" {
                break false;
            }

            if input.is_empty() || input == "yes" || input == "y" {
                break true;
            }
        };

        let subtraction = loop {
            println!("Enable Subtraction? (Y/n)");
            let input = get_input();
            if input == "no" || input == "n" {
                break false;
            }

            if input.is_empty() || input == "yes" || input == "y" {
                break true;
            }
        };

        let multiplication = loop {
            println!("Enable Multiplication? (Y/n)");
            let input = get_input();
            if input == "no" || input == "n" {
                break false;
            }

            if input.is_empty() || input == "yes" || input == "y" {
                break true;
            }
        };

        let division = loop {
            println!("Enable Division? (Y/n)");
            let input = get_input();
            if input == "no" || input == "n" {
                break false;
            }

            if input.is_empty() || input == "yes" || input == "y" {
                break true;
            }
        };

        if addition || subtraction || multiplication || division {
            break (addition, subtraction, multiplication, division);
        }
    }
}
