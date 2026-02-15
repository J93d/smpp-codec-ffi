# Contributing to smpp-codec-ffi

## Python Integration using Maturin

This project uses [Maturin](https://github.com/PyO3/maturin) to build and publish Python wheels.

### Prerequisites

- Rust (latest stable)
- Python 3.8+
- `maturin` installed (`pip install maturin`)

### Building for Development

To build and install the package in your current virtual environment:

```bash
maturin develop
```

### Building Wheels (Release)

To generate wheels for distribution:

```bash
maturin build --release
```

The wheels will be placed in `target/wheels/`.

### Cross-Compilation

To cross-compile for other platforms (e.g., from Linux to Windows), you typically run these commands within a Docker container or CI environment that has the necessary toolchains installed.

Refer to the [Maturin Documentation](https://www.maturin.rs/distribution.html) for detailed cross-compilation guides.
