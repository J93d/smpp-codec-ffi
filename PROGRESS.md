# Project Progress

Current status of `smpp-codec-ffi` implementation.

## Implementation Status

### Core Bindings
- [x] Common Types (Ton, Npi, BindMode)
- [x] Bind PDUs (Request/Response)
- [x] Submission PDUs (SubmitSm, SubmitMulti)
- [x] Delivery PDUs (DeliverSm, DeliveryReceipt)
- [x] Ancillary PDUs (EnquireLink, Unbind, AlertNotification, QuerySm, CancelSm, ReplaceSm)
- [x] Broadcast PDUs (BroadcastSm, QueryBroadcastSm, CancelBroadcastSm)
- [x] TLV Handling (Tags, Tlv)
- [x] Message Splitter (UDH, SAR, Payload)

### Language Bindings
- [x] Python (Maturin configuration and CI)
- [x] Java (Multi-threaded examples and JNA)
- [x] Kotlin (UniFFI generation)
- [x] Swift (Multi-threaded examples)

## Test Coverage
- [x] Total Coverage: **91.93%** (Exceeding 90% target)
- [x] Comprehensive tests for TLV, Common, and Splitter modules.
- [x] Unit tests for all major PDU types.

### Coverage Breakdown
- `src/ancillary_pdu_ffi.rs`: 87.39%
- `src/broadcast_pdu_ffi.rs`: 93.94%
- `src/common.rs`: 92.31% (Improved)
- `src/delivery_pdu_ffi.rs`: 95.77%
- `src/session_pdu_ffi.rs`: 93.44%
- `src/splitter.rs`: 87.50% (Improved)
- `src/submission_pdu_ffi.rs`: 96.45%
- `src/tlv.rs`: 92.31% (Improved)

## Future Goals
- [ ] Optimize memory copies between Rust and host languages.
- [ ] Add performance benchmarks for PDU encoding/decoding.
- [ ] Support for SMPP v5.0 specific TLVs more comprehensively.
