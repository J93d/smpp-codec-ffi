use smpp_codec_ffi::broadcast_pdu_ffi::*;
use smpp_codec_ffi::common::{Npi, Ton};


#[test]
fn test_broadcast_sm() {
    let req = BroadcastSmRequest {
        sequence_number: 100,
        service_type: "CMT".to_string(),
        source_addr_ton: Ton::International,
        source_addr_npi: Npi::Isdn,
        source_addr: "12345".to_string(),
        message_id: "msg_bcast".to_string(),
        priority_flag: 1,
        schedule_delivery_time: None,
        validity_period: None,
        replace_if_present_flag: 0,
        data_coding: 0,
        sm_default_msg_id: 0,
        optional_params: vec![],
    };

    let encoded = encode_broadcast_sm_request(&req);
    let decoded =
        decode_broadcast_sm_request(&encoded).expect("Failed to decode BroadcastSmRequest");
    assert_eq!(req, decoded);

    let resp = BroadcastSmResponse {
        sequence_number: 100,
        command_status: 0,
        status_description: "ESME_ROK".to_string(),
        message_id: "msg_bcast".to_string(),
        optional_params: vec![],
    };
    let encoded_resp = encode_broadcast_sm_response(&resp);
    let decoded_resp =
        decode_broadcast_sm_response(&encoded_resp).expect("Failed to decode BroadcastSmResponse");
    println!("Decoded BroadcastSmResponse: {:?}", decoded_resp);
    assert_eq!(resp, decoded_resp);
}
