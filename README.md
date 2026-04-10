# Rust Concepts Project

This project is a small Rust learning workspace organized by concept instead of
keeping every example inside a single `main.rs` file. The code is structured so
that each topic can be read independently and still be executed together from a
single binary.

## Project Structure

- `src/main.rs`: Thin executable entrypoint that runs each concept module.
- `src/lib.rs`: Crate-level module map for the learning examples.
- `src/basics.rs`: Variables, control flow, loops, stack and heap examples.
- `src/ownership.rs`: Ownership transfer, cloning, and returning ownership.
- `src/models.rs`: Shared structs and enums such as `User`, `Rectangle`, and `Shape`.
- `src/collections.rs`: Vectors, hash maps, `Option`, and basic `Result` handling.
- `src/iterators_demo.rs`: Immutable, mutable, consuming, and adapter-based iterators.
- `src/text.rs`: Strings, slices, and helper functions for first-word extraction.
- `src/abstractions.rs`: Generics, traits, trait bounds, and lifetime examples.
- `src/concurrency.rs`: Threads, move closures, and channels.
- `src/macros_demo.rs`: Formatting output with `Display`, `Debug`, and `println!`.

## How to Run

Use Cargo from the project root:

```bash
cargo run
```

To validate the code without running the examples:

```bash
cargo check
```

To build HTML documentation from the rustdoc comments added to each module:

```bash
cargo doc --open
```

## Documentation Approach

Each module now contains:

- A module-level rustdoc header explaining the concept being demonstrated.
- A public `run` function that acts as the module's entrypoint.
- Small, focused helper functions scoped to the concept they support.

This layout makes it easier to extend the project. New topics can be added as a
new module and then wired into `src/main.rs` without growing one large file.