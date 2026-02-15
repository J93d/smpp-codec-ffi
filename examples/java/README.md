# Java Examples

Comprehensive multi-threaded examples for Java.

## Prerequisites

- JDK 11+
- [JNA](https://github.com/java-native-access/jna) library.
- The compiled `smpp_codec_ffi.kt` (bridge) should be in the classpath.
- The shared library (`.so`, `.dylib`, or `.dll`) must be in the library path.

## Examples

-   **ToolClient.java**: Demonstrates a multi-threaded client sending multiple messages concurrently.
-   **ToolServer.java**: Demonstrates a multi-threaded server handling multiple connections.

## How to Run

1.  **Compile**:
    ```bash
    javac -cp ".:smpp-codec-ffi.jar:jna.jar" ToolServer.java ToolClient.java
    ```

2.  **Run Server**:
    ```bash
    java -cp ".:smpp-codec-ffi.jar:jna.jar" -Djava.library.path=. examples.java.ToolServer
    ```

3.  **Run Client**:
    ```bash
    java -cp ".:smpp-codec-ffi.jar:jna.jar" -Djava.library.path=. examples.java.ToolClient
    ```
