use smpp_codec_ffi::common::{Npi, Ton};
use smpp_codec_ffi::delivery_pdu_ffi::*;
use smpp_codec_ffi::tlv::tlv_new_u8;

#[test]
fn test_deliver_sm_encoding_decoding() {
    let request = DeliverSmRequest {
        sequence_number: 54321,
        service_type: "CMT".to_string(),
        source_addr_ton: Ton::International,
        source_addr_npi: Npi::Isdn,
        source_addr: "1234567890".to_string(),
        dest_addr_ton: Ton::National,
        dest_addr_npi: Npi::Isdn,
        dest_addr: "0987654321".to_string(),
        esm_class: 0,
        protocol_id: 0,
        priority_flag: 0,
        schedule_delivery_time: None,
        validity_period: None,
        registered_delivery: 1,
        replace_if_present_flag: 0,
        data_coding: 0,
        sm_default_msg_id: 0,
        short_message: b"Delivery Message".to_vec(),
        optional_params: vec![],
    };

    let encoded = encode_deliver_sm(&request);
    let decoded = decode_deliver_sm(&encoded).expect("Failed to decode DeliverSmRequest");

    assert_eq!(request, decoded);
}

#[test]
fn test_data_sm_encoding_decoding() {
    let tlv = tlv_new_u8(0x0424, 1); // Message Payload type or similar
    let request = DataSm {
        sequence_number: 99999,
        service_type: "DATA".to_string(),
        source_addr_ton: Ton::Alphanumeric,
        source_addr_npi: Npi::Unknown,
        source_addr: "DataSrc".to_string(),
        dest_addr_ton: Ton::Alphanumeric,
        dest_addr_npi: Npi::Unknown,
        dest_addr: "DataDest".to_string(),
        esm_class: 0,
        registered_delivery: 0,
        data_coding: 0,
        optional_params: vec![tlv],
    };

    let encoded = encode_data_sm(&request);
    let decoded = decode_data_sm(&encoded).expect("Failed to decode DataSm");

    assert_eq!(request, decoded);
}

#[test]
fn test_delivery_receipt_encoding() {
    let receipt = DeliveryReceipt {
        message_id: "msg123".to_string(),
        submitted_count: 1,
        delivered_count: 1,
        submit_date: "2206121200".to_string(),
        done_date: "2206121201".to_string(),
        status: "DELIVRD".to_string(),
        error_code: 0,
        text: "id:msg123 sub:001 dlvrd:001 submit date:2206121200 done date:2206121201 stat:DELIVRD err:000 text:".to_string(),
    };

    // Note: DeliveryReceipt doesn't have a decode function exposed in FFI, only encode to string bytes
    let encoded = encode_delivery_receipt(&receipt);
    let string_receipt = String::from_utf8(encoded).expect("Failed to convert receipt to string");

    assert!(string_receipt.contains("id:msg123"));
    assert!(string_receipt.contains("stat:DELIVRD"));
}

#[test]
fn test_deliver_sm_response() {
    let response = DeliverSmResponse {
        sequence_number: 12345,
        command_status: 0,
        message_id: "".to_string(),
        status_description: "ESME_ROK".to_string(),
    };

    let encoded = encode_delivery_sm_response(&response);
    let decoded =
        decode_delivery_sm_response(&encoded).expect("Failed to decode DeliverSmResponse");

    assert_eq!(response, decoded);
}

#[test]
fn test_data_sm_response() {
    let response = DataSmResponse {
        sequence_number: 9876,
        command_status: 0,
        message_id: "data_resp".to_string(),
        status_description: "ESME_ROK".to_string(),
        optional_params: vec![],
    };

    let encoded = encode_data_sm_response(&response);
    let decoded = decode_data_sm_response(&encoded).expect("Failed to decode DataSmResponse");

    assert_eq!(response, decoded);
}
