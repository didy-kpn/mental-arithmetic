#[derive(Default, Clone)]
pub struct Operators {
    addition: bool,
    subtraction: bool,
    multiplication: bool,
    division: bool,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl Operators {
    pub fn addition(&self) -> bool {
        self.addition
    }
    pub fn subtraction(&self) -> bool {
        self.subtraction
    }
    pub fn multiplication(&self) -> bool {
        self.multiplication
    }
    pub fn division(&self) -> bool {
        self.division
    }

    pub fn toggle_addition(&mut self) {
        self.addition = !self.addition;
    }

    pub fn toggle_subtraction(&mut self) {
        self.subtraction = !self.subtraction;
    }

    pub fn toggle_multiplication(&mut self) {
        self.multiplication = !self.multiplication;
    }

    pub fn toggle_division(&mut self) {
        self.division = !self.division;
    }
}
