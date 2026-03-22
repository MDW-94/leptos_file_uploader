use leptos::{component, view, IntoView};
use leptos::prelude::{ElementChild, ClassAttribute};
use stylance::import_style;

import_style!(style, "spin_icon.styled.css");

#[component]
pub fn SpinIcon() -> impl IntoView {
    view! {
        <svg class=style::spin_icon>
           <circle></circle>
            <path d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
        </svg>
    }
}