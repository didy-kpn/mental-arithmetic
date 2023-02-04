use yew::prelude::*;

mod components;
mod home;
mod module;
mod select_operators;
mod solve_arithmetic;

use home::Home;
use module::ModuleProvider;

#[function_component]
fn App() -> Html {
    html! {
        <ModuleProvider>
            <Home />
        </ModuleProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
