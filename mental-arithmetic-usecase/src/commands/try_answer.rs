use mental_arithmetic_domain::models::constant_value::ConstantValue;

pub struct TryAnswer(i32);

impl TryAnswer {
    pub fn new(answer: i32) -> Self {
        Self(answer)
    }
}

impl From<TryAnswer> for ConstantValue {
    fn from(answer: TryAnswer) -> ConstantValue {
        answer.0.into()
    }
}
