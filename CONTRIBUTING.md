# Contributing to Ethos

We are thrilled that you want to contribute to Ethos! Whether you are fixing a bug, improving the visualization, or adding a new protocol adapter, your help is appreciated.

## Development Workflow

1.  **Fork the repo**: Create your own copy of the repository.
2.  **Clone it**: `git clone git@github.com:One-Block-Org/Ethos.git`
3.  **Create a branch**: `git checkout -b feature/cool-new-thing`
4.  **Make changes**: Implement your logic.
5.  **Run tests**: `cargo test --workspace`
6.  **Check lints**: `cargo clippy --workspace` & `cargo fmt --all -- --check`
7.  **Submit a PR**: Open a Pull Request from your branch to our `main`.

## Code Standards

-   **Documentation**: Please document all public structs and functions using standard `///` doc comments.
-   **Tests**: All new features must include unit tests. Use `ethos-parser`'s existing tests as a template for trace-aggregation logic.
-   **Structure**: Keep the library and CLI logic separate.
    -   `ethos-core`: Shared types.
    -   `ethos-parser`: Pure trace transformation logic.
    -   `ethos-cli`: User-facing binary.

## License

By contributing, you agree that your contributions will be licensed under the project's [MIT](LICENSE-MIT) and [Apache 2.0](LICENSE-APACHE) licenses.
