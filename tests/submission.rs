use smpp_codec_ffi::common::{Npi, Ton};
use smpp_codec_ffi::submission_pdu_ffi::*;
use smpp_codec_ffi::tlv::tlv_new_u16;

#[test]
fn test_submit_sm_encoding_decoding() {
    let request = SubmitSmRequest {
        sequence_number: 12345,
        service_type: "CMT".to_string(),
        source_addr_ton: Ton::International,
        source_addr_npi: Npi::Isdn,
        source_addr: "1234567890".to_string(),
        dest_addr_ton: Ton::National,
        dest_addr_npi: Npi::Isdn,
        destination_addr: "0987654321".to_string(),
        esm_class: 0,
        protocol_id: 0,
        priority_flag: 1,
        schedule_delivery_time: None,
        validity_period: None,
        registered_delivery: 1,
        replace_if_present_flag: 0,
        data_coding: 0,
        sm_default_msg_id: 0,
        short_message: b"Hello, World!".to_vec(),
        tlvs: vec![],
    };

    let encoded = encode_submit_sm_request(&request);
    let decoded = decode_submit_sm_request(&encoded).expect("Failed to decode SubmitSmRequest");

    assert_eq!(request, decoded);
}

#[test]
fn test_submit_sm_with_tlvs() {
    let tlv = tlv_new_u16(0x1234, 0x5678);
    let request = SubmitSmRequest {
        sequence_number: 67890,
        service_type: "".to_string(),
        source_addr_ton: Ton::Unknown,
        source_addr_npi: Npi::Unknown,
        source_addr: "source".to_string(),
        dest_addr_ton: Ton::Unknown,
        dest_addr_npi: Npi::Unknown,
        destination_addr: "dest".to_string(),
        esm_class: 0,
        protocol_id: 0,
        priority_flag: 0,
        schedule_delivery_time: Some("220612123000000+".to_string()),
        validity_period: Some("220613123000000+".to_string()),
        registered_delivery: 0,
        replace_if_present_flag: 0,
        data_coding: 0,
        sm_default_msg_id: 0,
        short_message: vec![],
        tlvs: vec![tlv],
    };

    let encoded = encode_submit_sm_request(&request);
    let decoded = decode_submit_sm_request(&encoded).expect("Failed to decode SubmitSmRequest");

    assert_eq!(request, decoded);
}

#[test]
fn test_submit_multi_encoding_decoding() {
    let dest1 = Destination::SmeAddress {
        ton: Ton::International,
        npi: Npi::Isdn,
        address: "12345".to_string(),
    };
    let dest2 = Destination::DistributionList {
        name: "MyList".to_string(),
    };

    let request = SubmitMultiRequest {
        sequence_number: 11111,
        service_type: "".to_string(),
        source_addr_ton: Ton::Alphanumeric,
        source_addr_npi: Npi::Unknown,
        source_addr: "MyService".to_string(),
        destinations: vec![dest1, dest2],
        esm_class: 0,
        protocol_id: 0,
        priority_flag: 0,
        schedule_delivery_time: None,
        validity_period: None,
        registered_delivery: 0,
        replace_if_present_flag: 0,
        data_coding: 0,
        sm_default_msg_id: 0,
        short_message: b"Multi Broadcast".to_vec(),
        tlvs: vec![],
    };

    let encoded = encode_submit_multi_request(&request);
    let decoded =
        decode_submit_multi_request(&encoded).expect("Failed to decode SubmitMultiRequest");

    assert_eq!(request, decoded);
}

#[test]
fn test_submit_sm_response() {
    let response = SubmitSmResponse {
        sequence_number: 123,
        command_status: 0,
        message_id: "msg123".to_string(),
        status_description: "ESME_ROK".to_string(),
    };

    let encoded = encode_submit_sm_response(&response);
    let decoded = decode_submit_sm_response(&encoded).expect("Failed to decode SubmitSmResponse");

    assert_eq!(response, decoded);
}

#[test]
fn test_submit_multi_response() {
    let unsuccessful = UnsuccessfulDelivery {
        ton: Ton::International,
        npi: Npi::Isdn,
        address: "failed_addr".to_string(),
        error_status: 10,
    };

    let response = SubmitMultiResponse {
        sequence_number: 456,
        command_status: 0, // Partial success (ESME_ROK)
        message_id: "multi123".to_string(),
        status_description: "ESME_ROK".to_string(),
        unsuccess_smes: vec![unsuccessful],
    };

    let encoded = encode_submit_multi_response(&response);
    let decoded =
        decode_submit_multi_response(&encoded).expect("Failed to decode SubmitMultiResponse");

    assert_eq!(response, decoded);
}
