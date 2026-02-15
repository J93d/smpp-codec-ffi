use smpp_codec_ffi::common::SmppFfiError;
use smpp_codec_ffi::tlv::*;

#[test]
fn test_tlv_new_methods() {
    let tlv_u8 = tlv_new_u8(Tags::SAR_TOTAL_SEGMENTS, 5);
    assert_eq!(tlv_u8.tag, Tags::SAR_TOTAL_SEGMENTS);
    assert_eq!(tlv_u8.length, 1);
    assert_eq!(tlv_u8.value, vec![5]);

    let tlv_u16 = tlv_new_u16(Tags::SAR_MSG_REF_NUM, 1234);
    assert_eq!(tlv_u16.tag, Tags::SAR_MSG_REF_NUM);
    assert_eq!(tlv_u16.length, 2);
    assert_eq!(tlv_u16.value, vec![4, 210]); // 1234 in big endian

    let tlv_string = tlv_new_string(Tags::RECEIPTED_MESSAGE_ID, "msg123".to_string());
    assert_eq!(tlv_string.tag, Tags::RECEIPTED_MESSAGE_ID);
    assert_eq!(tlv_string.value, "msg123\0".as_bytes()); // C-string with null terminator

    let tlv_payload = tlv_new_payload(Tags::MESSAGE_PAYLOAD, vec![1, 2, 3]);
    assert_eq!(tlv_payload.tag, Tags::MESSAGE_PAYLOAD);
    assert_eq!(tlv_payload.value, vec![1, 2, 3]);
}

#[test]
fn test_tlv_from_name() {
    let tlv = tlv_new_u8_from_name("sar_total_segments", 10);
    assert_eq!(tlv.tag, Tags::SAR_TOTAL_SEGMENTS);
    assert_eq!(tlv.value, vec![10]);

    let tlv_u16 = tlv_new_u16_from_name("sar_msg_ref_num", 500);
    assert_eq!(tlv_u16.tag, Tags::SAR_MSG_REF_NUM);
    assert_eq!(tlv_u16.value, vec![1, 244]);
}

#[test]
fn test_tlv_encode_decode() {
    let tlv = tlv_new_u16(Tags::USER_MESSAGE_REFERENCE, 42);
    let bytes = tlv_encode(&tlv);

    // Tag (2) + Length (2) + Value (2) = 6 bytes
    assert_eq!(bytes.len(), 6);

    let decoded = tlv_decode(&bytes).unwrap();
    assert_eq!(decoded.tag, tlv.tag);
    assert_eq!(decoded.length, tlv.length);
    assert_eq!(decoded.value, tlv.value);
}

#[test]
fn test_tlv_value_conversions() {
    let tlv_u8 = tlv_new_u8(Tags::SAR_TOTAL_SEGMENTS, 2);
    assert_eq!(tlv_value_as_u8(&tlv_u8).unwrap(), 2);

    let tlv_u16 = tlv_new_u16(Tags::SAR_MSG_REF_NUM, 1000);
    assert_eq!(tlv_value_as_u16(&tlv_u16).unwrap(), 1000);

    let tlv_string = tlv_new_string(Tags::RECEIPTED_MESSAGE_ID, "test".to_string());
    assert_eq!(tlv_value_as_string(&tlv_string).unwrap(), "test");
}

#[test]
fn test_tlv_decode_errors() {
    let short_buffer = vec![0x00, 0x01]; // Too short for header
    let result = tlv_decode(&short_buffer);
    assert!(result.is_err());

    let invalid_data = vec![0x02, 0x01, 0x00, 0x05, 0x00]; // Length 5 but only 1 byte follows
    let result = tlv_decode(&invalid_data);
    assert!(result.is_err());
}

#[test]
fn test_tag_names() {
    assert_eq!(get_tag_by_name("sar_msg_ref_num"), Tags::SAR_MSG_REF_NUM);
    assert_eq!(
        get_tag_by_name("dest_addr_subunit"),
        Tags::DEST_ADDR_SUBUNIT
    );
}
