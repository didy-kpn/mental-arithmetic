use crate::{
    components::title::Title, module::ModuleContext, select_operators::SelectOperators,
    solve_arithmetic::SolveArithmetic,
};
use yew::prelude::*;

#[function_component]
pub fn Home() -> Html {
    let module = use_context::<ModuleContext>().unwrap();

    html! {
        <div class="flex flex-col items-center space-y-4">
            <Title />
            if module.scene.is_select_operators() {
                <SelectOperators />
            }
            if module.scene.is_solve_arithmetic() {
                <SolveArithmetic />
            }
            // <p>{s.to_string()}</p>
            // // <p>{if 0 < *s { html!{ <button>{"123"}</button> } } else { html!{ <p></p> } }}</p>
            // if 1 == *s { <button>{"123"}</button> } else { <p></p>  }
            // <button onclick={plus} class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 focus:outline-none font-medium py-2.5 px-5 mr-2 mb-2 rounded-lg text-sm">
            //   {"Button"}
            // </button>
        </div>
    }
}
