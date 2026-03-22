use leptos::ev::MouseEvent;
use leptos::prelude::*;
// use leptos::prelude::{component, view, Callable, Callback, IntoView, ReadSignal, Show, Signal};
use stylance::import_style;

use crate::components::icon::spin_icon::SpinIcon;

import_style!(style, "button.styled.css");

#[component]
pub fn Button(
    busy_reader: ReadSignal<bool>,
    #[prop(into)] _on_click: Callback<MouseEvent>,
    #[prop(default = "".to_string())] label: String,
    #[prop(default = "".to_string())] busy_label: String,
) -> impl IntoView {
    view! {
        <button 
            class=style::button_icon
            disabled=move || busy_reader.get()
            on:click=move |_ev| {
                if !busy_reader.get() {
                    // on_click.call(ev);
                }
            }>

        <Show when=move ||busy_reader.get()>
            <SpinIcon/>
        </Show>

        {move || if busy_reader.get() {
            busy_label.clone()
        } else {
            label.clone()
        }}
        </button>
    }
}