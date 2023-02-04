use yew::prelude::*;

#[function_component]
pub fn DivisionSign() -> Html {
    html! {
        <>
            <path fill="none" d="M0 0h24v24H0z"/>
            <path d="M5 11h14v2H5v-2zm7-3a1.5 1.5 0 1 1 0-3 1.5 1.5 0 0 1 0 3zm0 11a1.5 1.5 0 1 1 0-3 1.5 1.5 0 0 1 0 3z"/>
        </>
    }
}
