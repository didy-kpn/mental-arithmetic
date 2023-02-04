pub struct SolveArithmetic(bool);

impl SolveArithmetic {
    pub fn new(result: bool) -> Self {
        Self(result)
    }
}

impl SolveArithmetic {
    pub fn value(&self) -> bool {
        self.0
    }
}
