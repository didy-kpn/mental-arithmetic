use mental_arithmetic_domain::models::arithmetic::Arithmetic;
use mental_arithmetic_domain::repositories::arithmetic::ArithmeticRepository;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct InMemoryArithmeticClient(Rc<RefCell<Option<Arithmetic>>>);

impl InMemoryArithmeticClient {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(None)))
    }
}

impl Default for InMemoryArithmeticClient {
    fn default() -> Self {
        Self::new()
    }
}

impl ArithmeticRepository for InMemoryArithmeticClient {
    fn save(&self, arithmetic: Arithmetic) {
        *self.0.borrow_mut() = Some(arithmetic);
    }

    fn get(&self) -> Option<Arithmetic> {
        *self.0.clone().borrow()
    }
}

#[cfg(test)]
mod tests {
    use super::InMemoryArithmeticClient;
    use mental_arithmetic_domain::{
        models::{arithmetic::Arithmetic, operator::Operators},
        repositories::arithmetic::ArithmeticRepository,
    };

    #[test]
    fn test_inmemory_arithmetic_client() {
        let client = InMemoryArithmeticClient::default();
        assert!(client.get().is_none());

        let arithmetic = Arithmetic::new(Operators::new(
            true.into(),
            true.into(),
            true.into(),
            true.into(),
        ));
        client.save(arithmetic.clone());
        assert!(client.get().is_some());
        assert_eq!(
            client.get().unwrap().get_express(),
            arithmetic.get_express()
        );
        assert_eq!(client.get().unwrap().get_result(), arithmetic.get_result());
    }
}
