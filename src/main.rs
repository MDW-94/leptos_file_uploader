use leptos::prelude::*;
// use leptos::{component, view, IntoView};
// use leptos::leptos_dom::mount_to_body; Doesn't work for some reason, but the above does
 

#[component]
fn App() -> impl IntoView {
    view! {
        <p>Hello World</p>
    }
}


fn main(){
    mount_to_body(App);
}

