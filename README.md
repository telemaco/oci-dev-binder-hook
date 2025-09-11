# oci-dev-binder-hook
The OCI Device Binder is a generic OCI runtime hook that can be used with any container runtime that supports OCI hooks. It provides dynamic device access management based on container annotations.

## Building

To build the project, run the following command:

```bash
cargo build
```

## Testing

To run the test suite, use the following command:

```bash
cargo nextest run
```

## Development Environment

This project uses a few cargo extensions to ensure code quality and consistency. Please install the following tools:

*   **cargo-sort**: To check that `Cargo.toml` is sorted.
*   **cargo-machete**: To find unused dependencies.
*   **cargo-deny**: To check for security vulnerabilities and license compatibility.
*   **cargo-nextest**: For running tests.
*   **cargo-llvm-cov**: For generating code coverage reports.

You can install them using the following command:

```bash
cargo install cargo-sort cargo-machete cargo-deny cargo-nextest cargo-llvm-cov
```

## Code Coverage

To generate a code coverage report, you can use `cargo-llvm-cov`. It will use `cargo-nextest` under the hood.

First, ensure you have `cargo-llvm-cov` installed by following the instructions in the "Development Environment" section. You may also need to install the `llvm-tools-preview` component:
```bash
rustup component add llvm-tools-preview
```
Run the following command to obtain the coverage report

```bash
cargo llvm-cov nextest
```
