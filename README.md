# BookingSystem
Booking system project for A-Level CS

As for now, this project is on indefinite hold. The idea was to make a pure Rust web app that included database interaction using Yew and Postgres. Unfortunately, WASM does not support the crate `socket2` that `postgres` relies upon. I have considered using SurrealDB to make this stack even Rust-ier, however I cannot find any documentation for server-side management of it in pure Rust.
