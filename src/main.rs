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

// * mount_to_body comes from leptos library and is responsible for...
// TODO: Try and experiment with main function with mount_to_body within - what else can you do
// ! Remember to run
// ? Can you use Atomic Design for the folder structure?