# Rust Book

## General

Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed.

In Rust, packages of code are referred to as "crates".

## Commands

- `cargo new <project_name>`: initialize a new cargo project
- `cargo build`: compiles and creates an executable file in <target/debug/project_name>
- `cargo run`: compile the code and then run the resultant executable
- `cargo check`: checks the code to make sure it compiles but doesn’t produce an executable (**Much faster than `cargo build`**)
- `cargo build --release`: compiles the project with optimizations. Create an executable in <target/release> instead of <target/debug>

### Cargo

> Cargo comes installed with Rust if you used the official installers

---

Cargo is Rust’s build system and package manager.
Most Rustaceans use this tool to manage their Rust projects because Cargo handles a lot of tasks for you, such as building your code,
downloading the libraries your code depends on, and building those libraries.
(We call the libraries that your code needs dependencies.)

> Cargo expects your source files to live inside the src directory.
> The top-level project directory is just for README files,
> license information, configuration files, and anything else not related to your code.

---

## Formatting

If you want to stick to a standard style across Rust projects, you can use an automatic formatter tool called `rustfmt` to format your code in a particular style (more on rustfmt in Appendix D). The Rust team has included this tool with the standard Rust distribution, as rustc is, so it should already be installed on your compute
