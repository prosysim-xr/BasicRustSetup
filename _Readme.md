# Intro

- Basic Scafolding of Rust. Manually Do following to Get started.
- check out rust links `https://www.rust-lang.org/learn/get-started`

# Steps:

1. create a project structure like bellow
   MyApp
   |___ src
   |    |___ main.rs
   |    |___ lib.rs
   |    |___ my_random_files.rs
   |___ Cargo.toml
   |___ Readme.md
2. Or Use cargo to scafold a project using `>cargo new MyProjectApp`
3. Cargo.toml
   [package]
   name = "hello_world"
   version = "0.1.0"

   [dependencies]
   ferris-says = "0.2"
   Add some lib/package like this if you want to use it.
4. `>cargo build` to compile and check if errors and warning are not there.
5. `>cargo build`, `>cargo run` to build then run the app.

# Gotchas:

- snake case is prefered in rust lib/ or project  naming
