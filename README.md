# Leptos File Uploader

https://www.edinsonjim.idesoft.co/articles/building-a-file-uploader-using-leptos/

## How to Run

Check Rust is updated:

```bash
rustup update
```

Verify Rust version:

```bash
rustc --version
```

Install dependencies:

```
cargo install trunk
```

Run application

```
trunk serve --open
```

## Project Architecture

To maintain a clean, organized structure and follow the single responsibility principle, we will design the project with a simple directory architecture, this will help keep each part of our application modular and manageable. We will create three main directories:

- components: Contains reusable UI components that can be utilized across different parts of our application.
- pages: Holds the main page components, which serve as entry points for each view or route in the application.
- services: Houses modules dedicated to external API integration. We will implement the Service Agent Pattern to manage API calls and interactions with web services efficiently.

### Setting up the directory structure

Create the components, pages, and services directories, each containing a mod.rs file. The mod.rs file in each directory will alow us to expose these modules to the
rest of the application.Setting up the directory structure

Create the components, pages, and services directories, each containing a mod.rs file. The mod.rs file in each directory will alow us to expose these modules to the rest of the application.

In src/lib.rs, expose the submodules by adding the following code:

```rs
pub mod components;
pub mod pages;
pub mod services;
```

## To be continue [here](./src/components/README.md)

## Next Leptos project

https://autognosi.medium.com/building-a-modern-todo-list-application-with-rust-leptos-and-tailwind-css-4-0-28a859f4a17f

## CSS

Ref: https://github.com/basro/stylance-rs

## Full refresh of dependencies

```bash
rm -rf ~/.cargo/registry
rm -rf ~/.cargo/git
cargo clean
cargo build
```
