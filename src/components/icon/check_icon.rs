use leptos::{component, view, IntoView};
use leptos::prelude::{ElementChild, ClassAttribute};
use stylance::import_style;

import_style!(style, "check_icon.styled.css");

#[component]
pub fn CheckIcon() -> impl IntoView {
    view! {
        <svg class=style::check_icon>
            <path stroke-linecap="round" stroke-linejoin="round" d="m4.5 12.75 6 6 9-13.5" />
        </svg>
    }
}

// ! Your component signature is invalid: pub fn CheckIcon(#[prop()]) -> impl IntoView
// * Leptos requires every #[prop] field to have:
// * a name
// * a type
// ! right now you have neither
// ? Leptos expands #[component] into a function that expects well-formed props - an empty #[prop()] confuses the macro expansion and results in errors like: "expected identifier", "unexpected token", "failed to parse component props"

// * To fix: if the component has no props then remove (#[prop()])