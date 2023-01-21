# Setting up initial project
1. `cargo init` sets up the src dir and Cargo.toml
2. Use below to get all the necessary packages
   1. `cargo add actix-web`
   2. `cargo add serde_json`
   3. `cargo add serde --features derive`
3. `cargo run --release` (used to run the project)
4. `cargo build --release` (used to build the project)