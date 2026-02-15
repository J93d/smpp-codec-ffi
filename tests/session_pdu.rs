use smpp_codec_ffi::common::{BindMode, Npi, Ton};
use smpp_codec_ffi::session_pdu_ffi::*;

#[test]
fn test_bind_request() {
    let req = BindRequest {
        sequence_number: 1,
        mode: BindMode::Transceiver,
        system_id: "test".to_string(),
        password: "pass".to_string(),
        system_type: "type".to_string(),
        interface_version: 0x34,
        addr_ton: Ton::International,
        addr_npi: Npi::Isdn,
        address_range: "123".to_string(),
    };

    let encoded = encode_bind_request(&req);
    assert!(!encoded.is_empty());

    let decoded = decode_bind_request(&encoded).expect("Failed to decode BindRequest");
    assert_eq!(decoded.sequence_number, 1);
    assert_eq!(decoded.system_id, "test");
    assert_eq!(decoded.mode, BindMode::Transceiver);
}

#[test]
fn test_enquire_link() {
    let req = EnquireLink { sequence_number: 2 };
    let encoded = encode_enquire_link(&req);
    assert!(!encoded.is_empty());

    let decoded = decode_enquire_link(&encoded).expect("Failed to decode EnquireLink");
    assert_eq!(decoded.sequence_number, 2);
}

#[test]
fn test_outbind() {
    let req = Outbind {
        sequence_number: 3,
        system_id: "sys".to_string(),
    };
    let encoded = encode_outbind(&req);
    assert!(!encoded.is_empty());

    let decoded = decode_outbind(&encoded).expect("Failed to decode Outbind");
    assert_eq!(decoded.sequence_number, 3);
    assert_eq!(decoded.system_id, "sys");
}

#[test]
fn test_unbind() {
    let req = Unbind { sequence_number: 4 };
    let encoded = encode_unbind(&req);
    let decoded = decode_unbind(&encoded).expect("Failed to decode Unbind");
    assert_eq!(decoded.sequence_number, 4);
}

#[test]
fn test_generic_nack() {
    let req = GenericNack {
        sequence_number: 5,
        command_status: 0x00000001, // Invalid Message Length
    };
    let encoded = encode_generic_nack(&req);
    let decoded = decode_generic_nack(&encoded).expect("Failed to decode GenericNack");
    assert_eq!(decoded.sequence_number, 5);
    assert_eq!(decoded.command_status, 0x00000001);
}

#[test]
fn test_bind_response() {
    let resp = BindResponse {
        sequence_number: 6,
        command_status: 0,
        system_id: "server".to_string(),
    };
    let encoded = encode_bind_response(&resp);
    let decoded = decode_bind_response(&encoded).expect("Failed to decode BindResponse");
    assert_eq!(decoded.sequence_number, 6);
    assert_eq!(decoded.system_id, "server");
}

#[test]
fn test_enquire_link_response() {
    let resp = EnquireLinkResponse {
        sequence_number: 7,
        command_status: 0,
    };
    let encoded = encode_enquire_link_response(&resp);
    let decoded =
        decode_enquire_link_response(&encoded).expect("Failed to decode EnquireLinkResponse");
    assert_eq!(decoded.sequence_number, 7);
}

#[test]
fn test_unbind_response() {
    let resp = UnbindResponse {
        sequence_number: 8,
        command_status: 0,
    };
    let encoded = encode_unbind_response(&resp);
    let decoded = decode_unbind_response(&encoded).expect("Failed to decode UnbindResponse");
    assert_eq!(decoded.sequence_number, 8);
}
