# Swift Examples

This directory contains examples for using the `smpp-codec-ffi` Swift bindings.

## Prerequisites

1.  **Generated Bindings**: You need the `bindings/swift/` folder containing `.swift` files and module map.
2.  **Shared Library**: You need the compiled shared library (e.g., `libsmpp_codec_ffi.dylib` on macOS).

## Compilation and Running (macOS)

1.  Place the shared library in this directory.
2.  Compile:
    ```bash
    swiftc TestUsage.swift -I ../../bindings/swift -L. -lsmpp_codec_ffi -o test_usage
    ```
    (You might need to adjust `-I` to point to where the module map is).

### Tool Client/Server

1.  **Compile**:
    ```bash
    swiftc ToolServer.swift -I ../../bindings/swift -L. -lsmpp_codec_ffi -o tool_server
    swiftc ToolClient.swift -I ../../bindings/swift -L. -lsmpp_codec_ffi -o tool_client
    ```

2.  **Run Server**:
    ```bash
    ./tool_server
    ```

3.  **Run Client**:
    ```bash
    ./tool_client
    ```
