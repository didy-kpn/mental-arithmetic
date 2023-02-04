pub mod arithmetic;
pub mod operators;
pub mod scene;

use std::rc::Rc;

use uuid::Uuid;
use yew::prelude::*;

use self::{
    arithmetic::Arithmetic,
    operators::{Operator, Operators},
    scene::Scene,
};

pub type ModuleContext = UseReducerHandle<Module>;

#[derive(Clone)]
pub struct Module {
    _id: Uuid,
    pub arithmetic: Arithmetic,
    pub operators: Operators,
    pub scene: Scene,
}

#[derive(PartialEq, Eq, Clone)]
pub enum ModuleAction {
    Toggle(Operator),
    Move(Scene),
}

impl Reducible for Module {
    type Action = ModuleAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let arithmetic = self.arithmetic.clone();
        let mut operators = self.operators.clone();
        let mut scene = self.scene.clone();
        match action {
            ModuleAction::Toggle(Operator::Addition) => operators.toggle_addition(),
            ModuleAction::Toggle(Operator::Subtraction) => operators.toggle_subtraction(),
            ModuleAction::Toggle(Operator::Multiplication) => operators.toggle_multiplication(),
            ModuleAction::Toggle(Operator::Division) => operators.toggle_division(),
            ModuleAction::Move(next_scene) => scene = next_scene,
        }
        Module::new(arithmetic, operators, scene).into()
    }
}

impl PartialEq for Module {
    fn eq(&self, other: &Self) -> bool {
        self._id == other._id
    }
}

impl Eq for Module {}

impl Default for Module {
    fn default() -> Self {
        Self {
            _id: Uuid::new_v4(),
            arithmetic: Arithmetic::default(),
            operators: Operators::default(),
            scene: Scene::default(),
        }
    }
}

impl Module {
    fn new(arithmetic: Arithmetic, operators: Operators, scene: Scene) -> Self {
        Self {
            _id: Uuid::new_v4(),
            arithmetic,
            operators,
            scene,
        }
    }
}

#[derive(Properties, Debug, PartialEq)]
pub struct ModuleProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn ModuleProvider(props: &ModuleProviderProps) -> Html {
    let module = use_reducer(Module::default);

    html! {
        <ContextProvider<ModuleContext> context={module}>
            {props.children.clone()}
        </ContextProvider<ModuleContext>>
    }
}
