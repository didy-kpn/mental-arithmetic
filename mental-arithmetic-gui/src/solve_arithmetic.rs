use crate::{
    components::button_link::ButtonLink,
    module::{scene::Scene, ModuleAction, ModuleContext},
};
use yew::prelude::*;

use web_sys::HtmlTextAreaElement;

#[function_component]
pub fn SolveArithmetic() -> Html {
    let module = use_context::<ModuleContext>().unwrap();
    let arithmetic = module.arithmetic.clone();
    let operators = module.operators.clone();

    let express = use_state(|| {
        arithmetic.create(
            operators.addition(),
            operators.subtraction(),
            operators.multiplication(),
            operators.division(),
        )
    });

    let correct = use_mut_ref(|| 0);
    let incorrect = use_mut_ref(|| 0);
    let input = use_mut_ref(String::new);
    let oninput = {
        let arithmetic = arithmetic.clone();
        let input = input.clone();
        let express = express.clone();
        let correct = correct.clone();
        let incorrect = incorrect.clone();
        Callback::from(move |e: InputEvent| {
            let v: HtmlTextAreaElement = e.target_unchecked_into();
            *input.borrow_mut() = v.value();

            let answer = (*input.borrow()).to_string();
            if answer.len() != arithmetic.result().to_string().len() {
                return;
            }

            if arithmetic.solve(answer.parse().unwrap()) {
                *correct.borrow_mut() += 1;
            } else {
                *incorrect.borrow_mut() += 1;
            }

            *input.borrow_mut() = "".to_string();
            express.set(arithmetic.create(
                operators.addition(),
                operators.subtraction(),
                operators.multiplication(),
                operators.division(),
            ));
        })
    };

    let link = Callback::from(move |_| {
        module.dispatch(ModuleAction::Move(Scene::SelectOperators));
    });

    html! {
        <>
            <div class="text-xl font-bold">{format!("{} {} {} = ?", express.0, express.1, express.2)}</div>

            <input value={input.borrow().clone()} {oninput} autofocus=true min=0 max=1000 type="number" class="bg-white focus:outline-none focus:shadow-outline border border-gray-300 rounded-lg py-2 px-4 block w-full appearance-none leading-normal" />

            <table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
                <tbody>
                    <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600">
                        <th scope="row" class="py-4 px-6 font-medium text-gray-900 whitespace-nowrap dark:text-white">
                            <p class="text-xl font-bold">{"Correct"}</p>
                        </th>
                        <td class="py-4 px-6">
                            <p class="text-xl font-bold">{correct.borrow().to_string()}</p>
                        </td>
                    </tr>
                    <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600">
                        <th scope="row" class="py-4 px-6 font-medium text-gray-900 whitespace-nowrap dark:text-white">
                            <p class="text-xl font-bold">{"Incorrect"}</p>
                        </th>
                        <td class="py-4 px-6">
                            <p class="text-xl font-bold">{incorrect.borrow().to_string()}</p>
                        </td>
                    </tr>
                </tbody>
            </table>

            <ButtonLink name={"Back"} onclick={link} />
        </>
    }
}
