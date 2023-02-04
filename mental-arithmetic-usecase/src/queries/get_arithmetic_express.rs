use mental_arithmetic_domain::models::arithmetic::Arithmetic;

pub struct GetArithmeticExpress {
    left_value: i32,
    operator: String,
    right_value: i32,
}

impl GetArithmeticExpress {
    pub fn new(arithmetic: Arithmetic) -> Self {
        let (left_value, operator, right_value) = arithmetic.get_express();

        Self {
            left_value: left_value.into(),
            operator: operator.into(),
            right_value: right_value.into(),
        }
    }
}

impl GetArithmeticExpress {
    pub fn left_value(&self) -> i32 {
        self.left_value
    }

    pub fn operator(&self) -> String {
        self.operator.clone()
    }

    pub fn right_value(&self) -> i32 {
        self.right_value
    }
}
