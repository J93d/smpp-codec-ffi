use smpp_codec_ffi::splitter::*;

#[test]
fn test_split_message_gsm7bit() {
    let text = "Hello World".to_string();
    let result = split_message(text, EncodingType::Gsm7Bit, SplitMode::Payload)
        .expect("Failed to split message");

    assert_eq!(result.parts.len(), 1);
    assert_eq!(result.data_coding, 0x00); // GSM 7-bit
}

#[test]
fn test_split_message_ucs2() {
    let text = "Hello World 🚀".to_string(); // Contains emoji, needs UCS2
    let result = split_message(text, EncodingType::Ucs2, SplitMode::Payload)
        .expect("Failed to split message");

    // Check if it decided to use UCS2 (data_coding 0x08)
    assert_eq!(result.data_coding, 0x08);
}

#[test]
fn test_split_message_concat_sar() {
    // Create a long string > 160 chars (GSM 7-bit limit) or > 140 bytes
    let text = "A".repeat(1000);
    // GSM 7-bit: 200 chars.
    // If we force UDH or SAR, it should split.
    // SAR mode
    let result = split_message(text.clone(), EncodingType::Latin1, SplitMode::Sar)
        .expect("Failed to split message");

    println!(
        "Parts: {}, len[0]: {}, dcs: {}",
        result.parts.len(),
        result.parts[0].len(),
        result.data_coding
    );

    assert!(result.parts.len() > 1);
    assert_eq!(result.data_coding, 0x03);
}
