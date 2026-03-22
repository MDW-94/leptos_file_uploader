# Components

To make our application modular and reusable, we will create reusable components, like icons and buttons.

## Icon Components

Our application requires icon components to represent some UI elements visually. We will create an icons module within the components module to organize these icons, using SVG format. You can utilize icons iconoir, heroicons or any other preferred library if desired.

To organize our icons, create the following structure within the components directory.

```
.
├── src
│   ├── components
│   │   ├── icons
│   │   │   │── check_icon.rs
│   │   │   │── info_icon.rs
│   │   │   │── mod.rs
│   │   │   │── spin_icon.rs
│   │   │   └── trash_icon.rs
│   │   └── mod.rs
```

## Check icon component

...

## Buttons Components

Our app will include two types of buttons: a simple text button and a button with an icon. To organize them, we will create a 'buttons' module within the 'components' modules.

```
├── components
│   ├── buttons
│   │   ├── button.rs
│   │   ├── button_icon.rs
│   │   └── mod.rs
│   └── mod.rs
├── lib.rs
```

## Leptos Component Debug Comment

The #[component] macro allows you to annotate plain Rust functions as components and use them within your Leptos view as if they were custom HTML elements. The component function takes any number of other arguments. When you use the component somewhere else, the names of its arguments are the names of the properties you use in the view macro.

Every component function should have the return type -> impl IntoView.

You can add Rust doc comments to component function arguments and the macro will use them to generate documentation for the component.

Here’s how you would define and use a simple Leptos component which can accept custom properties for a name and age:

### Continued:

```rust

#[component]
pub fn MyComponent(
    #[prop(into)] name: String,
    #[prop(optional)] optional_value: Option<i32>,
    #[prop(optional_no_strip)] optional_no_strip: Option<i32>,
    #[prop(default = 7)] optional_default: i32,
    #[prop(name = "data")] UserInfo { email, user_id }: UserInfo,
) -> impl IntoView {
    // whatever UI you need
}
```

## proc-macro error

```
proc-macro panicked: failed to load macro: Cannot create expander for /Users/apple/Dev/projects/rust/leptos/file-uploader-leptos/target/debug/deps/libleptos_macro-ff5a1ea11260052b.dylib: mismatched ABI expected: `rustc 1.96.0-nightly (03749d625 2026-03-14)`, got `rustc 1.96.0-nightly (20f19f461 2026-03-21)`
```

That error looks dramatic, but the root cause is actually very down‑to‑earth: your Leptos procedural macro was compiled with one nightly Rust version, and your project is now being compiled with a different nightly version.

Rust nightly is not ABI‑stable, so any proc‑macro compiled under one nightly cannot be loaded by a different nightly—even if the version numbers look close.

Here’s the key part of your error:

Code

```
mismatched ABI expected: `rustc 1.96.0-nightly (03749d625 2026-03-14)`,
got `rustc 1.96.0-nightly (20f19f461 2026-03-21)`
```

Your project is using the 2026‑03‑21 nightly, but the Leptos macro dylib in target/ was built with the 2026‑03‑14 nightly.

When the compiler tries to load the macro, it panics because the ABI doesn’t match.

1. Clean your build artifacts
   This forces all proc‑macros to be rebuilt with the current nightly.

```bash
cargo clean
```

Then rebuild:

```bash
cargo build
```

This alone fixes it in most cases.
