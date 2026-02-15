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
- [x] Total Coverage: ~92.4% (Increased from 84.6%)
- [x] Comprehensive tests for TLV, Common, and Splitter modules.
- [x] Unit tests for all major PDU types.

### Coverage Breakdown
- `src/ancillary_pdu_ffi.rs`: 87.39% (208/238)
- `src/broadcast_pdu_ffi.rs`: 93.94% (62/66)
- `src/common.rs`: 50% (26/52) - *Lower due to many enum conversions and constants.*
- `src/delivery_pdu_ffi.rs`: 95.77% (136/142)
- `src/session_pdu_ffi.rs`: 93.44% (114/122)
- `src/splitter.rs`: 54.17% (13/24) - *Needs more FFI-specific unit tests.*
- `src/submission_pdu_ffi.rs`: 96.45% (163/169)
- `src/tlv.rs`: 23.08% (12/52) - *Needs extensive TLV creation/encode/decode tests.*

## Future Goals
- [ ] Reach 90%+ overall code coverage.
- [ ] Improve `src/tlv.rs` coverage with more robust edge-case tests.
- [ ] Add more comprehensive multi-threading examples for Kotlin and Swift.
- [ ] Optimize memory copies between Rust and host languages.
