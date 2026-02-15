//! FFI bindings for smpp-codec.

#![warn(missing_docs)]
uniffi::setup_scaffolding!();
/// Ancillary PDU FFI module.
pub mod ancillary_pdu_ffi;
/// Broadcast PDU FFI module.
pub mod broadcast_pdu_ffi;
/// Common types FFI module.
pub mod common;
/// Delivery PDU FFI module.
pub mod delivery_pdu_ffi;
/// Session PDU FFI module.
pub mod session_pdu_ffi;
/// Splitter FFI module.
pub mod splitter;
/// Submission PDU FFI module.
pub mod submission_pdu_ffi;
/// TLV FFI module.
pub mod tlv;
