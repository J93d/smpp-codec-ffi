# Python Examples

This directory contains examples for using the `smpp-codec-ffi` Python bindings.

## Prerequisites

1.  **Install the package**:
    You need to install the generated `.whl` file.
    
    ```bash
    pip install /path/to/smpp_codec_ffi-0.1.0-cp37-abi3-manylinux_2_24_x86_64.whl
    ```
    
    (The wheel file is available in the GitHub Actions artifacts under `python-wheels`).

## Running the Examples

### Basic Test
Once installed, simply run the test script:

```bash
python test_usage.py
```

### Tool Client/Server (Full PDU Sequence)

1.  **Start the Server**:
    ```bash
    python tool_server.py
    ```

2.  **Run the Client** (in a separate terminal):
    ```bash
    python tool_client.py
    ```
