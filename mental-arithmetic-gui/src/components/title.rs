use yew::prelude::*;

#[function_component]
pub fn Title() -> Html {
    html! {
        <>
            <h1 class="mb-4 text-4xl font-extrabold tracking-tight leading-none text-gray-900 md:text-5xl lg:text-6xl dark:text-white">{"Mental Arithmetic"}</h1>
        </>
    }
}
