use yew::prelude::*;

#[derive(PartialEq, Clone, Properties)]
pub struct LinkProps {
    pub name: String,
    pub onclick: Callback<MouseEvent>,
}

#[function_component]
pub fn ButtonLink(props: &LinkProps) -> Html {
    html! {
        <>
            <button onclick={props.onclick.clone()} class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 focus:outline-none font-medium py-2.5 px-5 mr-2 mb-2 rounded-lg text-sm">
              {props.name.clone()}
            </button>

        </>
    }
}
