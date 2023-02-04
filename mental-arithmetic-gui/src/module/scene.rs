#[derive(Default, PartialEq, Eq, Clone)]
pub enum Scene {
    #[default]
    SelectOperators,
    SolveArithmetic,
}

impl Scene {
    pub fn is_select_operators(&self) -> bool {
        *self == Scene::SelectOperators
    }

    pub fn is_solve_arithmetic(&self) -> bool {
        *self == Scene::SolveArithmetic
    }
}
