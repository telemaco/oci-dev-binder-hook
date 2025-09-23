# oci-dev-binder-hook
The OCI Device Binder is a generic OCI runtime hook that can be used with any container runtime that supports OCI hooks. It provides dynamic device access management based on container annotations.

## Usage

The `oci-dev-binder-hook` can be used to enrich a container's available devices by using OCI annotations.

### With Quadlets

For example, you can use a quadlet file for `podman` to inject all the devices that belongs to a given seat.

To do so, you can create a file `/etc/containers/systemd/mycontainer.container` with the following content:

```
[Container]
Image=<image>
Annotation=io.dev-binder.udev.seat=seat0
```

Then, when you run a container by enabling the `mycontainer.service` systemd unit, the hook will inject all the devices that belongs to the `seat0` seat into the container.

### With Podman CLI

Alternatively, you can achieve the same result using the `podman` command-line interface directly:

```bash
podman run --annotation io.dev-binder.udev.seat=seat0 -it <image>
```

This command will trigger the hook, which will then inject all the devices associated with the `seat0` seat into the container.

## Building

To build the project, run the following command:

```bash
cargo build
```

### Building with Meson

It is also possible to build the project using Meson. This is useful for packagers and developers who want to integrate the project with other Meson-based projects.

First, ensure you have Meson installed. You can find installation instructions on the [Meson website](https://mesonbuild.com/Getting-meson.html).

Once you have Meson installed, you can build the project using the following commands:

```bash
meson setup build --prefix=/usr
meson compile -C build
```

To install the project, you can use the following command:

```bash
meson install -C build
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
