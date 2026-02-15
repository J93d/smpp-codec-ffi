use smpp_codec_ffi::ancillary_pdu_ffi::*;
use smpp_codec_ffi::common::{Npi, Ton};


#[test]
fn test_cancel_sm() {
    let req = CancelSmRequest {
        sequence_number: 1,
        service_type: "CMT".to_string(),
        message_id: "msg1".to_string(),
        source_addr_ton: Ton::International,
        source_addr_npi: Npi::Isdn,
        source_addr: "123".to_string(),
        dest_addr_ton: Ton::National,
        dest_addr_npi: Npi::Isdn,
        dest_addr: "456".to_string(),
    };

    let encoded = encode_cancel_sm_request(&req);
    let decoded = decode_cancel_sm_request(&encoded).expect("Failed to decode CancelSmRequest");
    assert_eq!(req, decoded);

    let resp = CancelSmResponse {
        sequence_number: 1,
        command_status: 0,
        status_description: "ESME_ROK".to_string(),
    };
    let encoded_resp = encode_cancel_sm_response(&resp);
    let decoded_resp =
        decode_cancel_sm_response(&encoded_resp).expect("Failed to decode CancelSmResponse");
    println!("Decoded CancelSmResponse: {:?}", decoded_resp);
    assert_eq!(resp, decoded_resp);
}

#[test]
fn test_query_sm() {
    let req = QuerySmRequest {
        sequence_number: 2,
        message_id: "msg2".to_string(),
        source_addr_ton: Ton::Alphanumeric,
        source_addr_npi: Npi::Unknown,
        source_addr: "Sender".to_string(),
    };

    let encoded = encode_query_sm_request(&req);
    let decoded = decode_query_sm_request(&encoded).expect("Failed to decode QuerySmRequest");
    assert_eq!(req, decoded);

    let resp = QuerySmResponse {
        sequence_number: 2,
        command_status: 0,
        message_id: "msg2".to_string(),
        final_date: "2206121200".to_string(),
        message_state: 2, // Delivered
        error_code: 0,
        status_description: "ESME_ROK".to_string(),
    };
    let encoded_resp = encode_query_sm_response(&resp);
    let decoded_resp =
        decode_query_sm_response(&encoded_resp).expect("Failed to decode QuerySmResponse");
    assert_eq!(resp, decoded_resp);
}

#[test]
fn test_replace_sm() {
    let req = ReplaceSmRequest {
        sequence_number: 3,
        message_id: "msg3".to_string(),
        source_addr_ton: Ton::International,
        source_addr_npi: Npi::Isdn,
        source_addr: "123".to_string(),
        schedule_delivery_time: "".to_string(),
        validity_period: "".to_string(),
        registered_delivery: 0,
        sm_default_msg_id: 0,
        short_message: b"New Message".to_vec(),
    };

    let encoded = encode_replace_sm_request(&req);
    let decoded = decode_replace_sm_request(&encoded).expect("Failed to decode ReplaceSmRequest");
    assert_eq!(req, decoded);

    let resp = ReplaceSmResponse {
        sequence_number: 3,
        command_status: 0,
        status_description: "ESME_ROK".to_string(),
    };
    let encoded_resp = encode_replace_sm_response(&resp);
    let decoded_resp =
        decode_replace_sm_response(&encoded_resp).expect("Failed to decode ReplaceSmResponse");
    assert_eq!(resp, decoded_resp);
}

#[test]
fn test_cancel_broadcast_sm() {
    let req = CancelBroadcastSmRequest {
        sequence_number: 4,
        service_type: "CBM".to_string(),
        message_id: "bcast1".to_string(),
        source_addr_ton: Ton::International,
        source_addr_npi: Npi::Isdn,
        source_addr: "123".to_string(),
        optional_params: vec![],
    };

    let encoded = encode_cancel_broadcast_sm_request(&req);
    let decoded = decode_cancel_broadcast_sm_request(&encoded)
        .expect("Failed to decode CancelBroadcastSmRequest");
    assert_eq!(req, decoded);

    let resp = CancelBroadcastSmResponse {
        sequence_number: 4,
        command_status: 0,
        status_description: "ESME_ROK".to_string(),
    };
    let encoded_resp = encode_cancel_broadcast_sm_response(&resp);
    let decoded_resp = decode_cancel_broadcast_sm_response(&encoded_resp)
        .expect("Failed to decode CancelBroadcastSmResponse");
    assert_eq!(resp, decoded_resp);
}

#[test]
fn test_query_broadcast_sm() {
    let req = QueryBroadcastSmRequest {
        sequence_number: 5,
        message_id: "bcast2".to_string(),
        source_addr_ton: Ton::International,
        source_addr_npi: Npi::Isdn,
        source_addr: "123".to_string(),
        optional_params: vec![],
    };

    let encoded = encode_query_broadcast_sm_request(&req);
    let decoded = decode_query_broadcast_sm_request(&encoded)
        .expect("Failed to decode QueryBroadcastSmRequest");
    assert_eq!(req, decoded);

    let resp = QueryBroadcastSmResponse {
        sequence_number: 5,
        command_status: 0,
        status_description: "ESME_ROK".to_string(),
        message_id: "bcast2".to_string(),
        optional_params: vec![],
    };
    let encoded_resp = encode_query_broadcast_sm_response(&resp);
    let decoded_resp = decode_query_broadcast_sm_response(&encoded_resp)
        .expect("Failed to decode QueryBroadcastSmResponse");
    assert_eq!(resp, decoded_resp);
}
