use yew::prelude::*;

use crate::{
    components::{
        button_link::ButtonLink,
        li_item::LiItem,
        svg::{
            addition::AdditionSign, division::DivisionSign, multiplication::MultiplicationSign,
            substraction::SubstractionSign,
        },
    },
    module::{operators::Operator, scene::Scene, ModuleAction, ModuleContext},
};

#[function_component]
pub fn SelectOperators() -> Html {
    let module = use_context::<ModuleContext>().unwrap();
    let operators = module.operators.clone();

    let enabled = operators.addition()
        || operators.subtraction()
        || operators.multiplication()
        || operators.division();

    let toggle_addition = {
        let module = module.clone();
        Callback::from(move |_| {
            module.dispatch(ModuleAction::Toggle(Operator::Addition));
        })
    };

    let toggle_subtraction = {
        let module = module.clone();
        Callback::from(move |_| {
            module.dispatch(ModuleAction::Toggle(Operator::Subtraction));
        })
    };

    let toggle_multiplication = {
        let module = module.clone();
        Callback::from(move |_| {
            module.dispatch(ModuleAction::Toggle(Operator::Multiplication));
        })
    };

    let toggle_division = {
        let module = module.clone();
        Callback::from(move |_| {
            module.dispatch(ModuleAction::Toggle(Operator::Division));
        })
    };

    let link = Callback::from(move |_| {
        module.dispatch(ModuleAction::Move(Scene::SolveArithmetic));
    });

    html! {
        <>
            <ul class="grid gap-6 w-full md:grid-cols-2">
                <LiItem
                    name={"Addition"}
                    color={"sky"}
                    checked={operators.addition()}
                    onclick={toggle_addition}
                >
                    <AdditionSign />
                </LiItem>
                <LiItem
                    name={"Subtraction"}
                    color={"green"}
                    checked={operators.subtraction()}
                    onclick={toggle_subtraction}
                >
                    <SubstractionSign />
                </LiItem>
                <LiItem
                    name={"Multiplication"}
                    color={"red"}
                    checked={operators.multiplication()}
                    onclick={toggle_multiplication}
                >
                    <MultiplicationSign />
                </LiItem>

                <LiItem
                    name={"Division"}
                    color={"yellow"}
                    checked={operators.division()}
                    onclick={toggle_division}
                >
                    <DivisionSign />
                </LiItem>
            </ul>

            if enabled {
                <ButtonLink name={"Start"} onclick={link} />
            }
        </>
    }
}
