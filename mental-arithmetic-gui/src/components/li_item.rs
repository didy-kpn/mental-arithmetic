use yew::prelude::*;

#[derive(PartialEq, Clone, Properties)]
pub struct CheckItemProps {
    pub name: String,
    pub checked: bool,
    pub onclick: Callback<MouseEvent>,
    pub color: String,
    pub children: Children,
}

impl CheckItemProps {
    pub fn id(&self) -> String {
        format!("{}-option", self.name.to_lowercase())
    }

    pub fn color(&self) -> String {
        format!("mb-2 w-10 h-10 text-{}-500", self.color)
    }
}

#[function_component]
pub fn LiItem(props: &CheckItemProps) -> Html {
    html! {
        <li>
            <input type="checkbox" checked={props.checked} onclick={props.onclick.clone()} id={props.id()} class="hidden peer" />
            <label for={props.id()} class="inline-flex justify-between items-center p-5 w-full text-gray-500 bg-white rounded-lg border-2 border-gray-200 cursor-pointer dark:hover:text-gray-300 dark:border-gray-700 peer-checked:border-blue-600 hover:text-gray-600 dark:peer-checked:text-gray-300 peer-checked:text-gray-600 hover:bg-gray-50 dark:text-gray-400 dark:bg-gray-800 dark:hover:bg-gray-700">
                <div class="block">
                    <svg class={props.color()} fill="currentColor" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                        {for props.children.iter()}
                    </svg>
                    <div class="w-full text-lg font-semibold">{props.name.clone()}</div>
                </div>
            </label>
        </li>
    }
}
