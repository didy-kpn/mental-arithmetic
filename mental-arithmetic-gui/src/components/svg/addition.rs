use yew::prelude::*;

#[function_component]
pub fn AdditionSign() -> Html {
    html! {
        <>
            <path fill="none" d="M0 0h24v24H0z"/>
            <path d="M11 11V5h2v6h6v2h-6v6h-2v-6H5v-2z"/>
        </>
    }
}
