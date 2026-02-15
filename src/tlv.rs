use smpp_codec::tlv::tags as native_tags;
use smpp_codec::tlv::Tlv as NativeTlv;
use uniffi;

/// Collection of standard SMPP TLV tags.
#[derive(uniffi::Object)]
pub struct Tags;

impl Tags {
    /// Destination address subunit.
    pub const DEST_ADDR_SUBUNIT: u16 = native_tags::DEST_ADDR_SUBUNIT;
    /// Destination network type.
    pub const DEST_NETWORK_TYPE: u16 = native_tags::DEST_NETWORK_TYPE;
    /// Destination bearer type.
    pub const DEST_BEARER_TYPE: u16 = native_tags::DEST_BEARER_TYPE;
    /// Destination telematics ID.
    pub const DEST_TELEMATICS_ID: u16 = native_tags::DEST_TELEMATICS_ID;
    /// Source address subunit.
    pub const SOURCE_ADDR_SUBUNIT: u16 = native_tags::SOURCE_ADDR_SUBUNIT;
    /// Source network type.
    pub const SOURCE_NETWORK_TYPE: u16 = native_tags::SOURCE_NETWORK_TYPE;
    /// Source bearer type.
    pub const SOURCE_BEARER_TYPE: u16 = native_tags::SOURCE_BEARER_TYPE;
    /// Source telematics ID.
    pub const SOURCE_TELEMATICS_ID: u16 = native_tags::SOURCE_TELEMATICS_ID;
    /// QoS time to live.
    pub const QOS_TIME_TO_LIVE: u16 = native_tags::QOS_TIME_TO_LIVE;
    /// Payload type.
    pub const PAYLOAD_TYPE: u16 = native_tags::PAYLOAD_TYPE;
    /// Additional status info text.
    pub const ADDITIONAL_STATUS_INFO_TEXT: u16 = native_tags::ADDITIONAL_STATUS_INFO_TEXT;
    /// Receipted message ID.
    pub const RECEIPTED_MESSAGE_ID: u16 = native_tags::RECEIPTED_MESSAGE_ID;
    /// MS message wait facilities.
    pub const MS_MSG_WAIT_FACILITIES: u16 = native_tags::MS_MSG_WAIT_FACILITIES;
    /// Privacy indicator.
    pub const PRIVACY_INDICATOR: u16 = native_tags::PRIVACY_INDICATOR;
    /// Source subaddress.
    pub const SOURCE_SUBADDRESS: u16 = native_tags::SOURCE_SUBADDRESS;
    /// Destination subaddress.
    pub const DEST_SUBADDRESS: u16 = native_tags::DEST_SUBADDRESS;
    /// User message reference.
    pub const USER_MESSAGE_REFERENCE: u16 = native_tags::USER_MESSAGE_REFERENCE;
    /// User response code.
    pub const USER_RESPONSE_CODE: u16 = native_tags::USER_RESPONSE_CODE;
    /// Source port.
    pub const SOURCE_PORT: u16 = native_tags::SOURCE_PORT;
    /// Destination port.
    pub const DESTINATION_PORT: u16 = native_tags::DESTINATION_PORT;
    /// SAR message reference number.
    pub const SAR_MSG_REF_NUM: u16 = native_tags::SAR_MSG_REF_NUM;
    /// Language indicator.
    pub const LANGUAGE_INDICATOR: u16 = native_tags::LANGUAGE_INDICATOR;
    /// SAR total segments.
    pub const SAR_TOTAL_SEGMENTS: u16 = native_tags::SAR_TOTAL_SEGMENTS;
    /// SAR segment sequence number.
    pub const SAR_SEGMENT_SEQNUM: u16 = native_tags::SAR_SEGMENT_SEQNUM;
    /// SC interface version.
    pub const SC_INTERFACE_VERSION: u16 = native_tags::SC_INTERFACE_VERSION;
    /// Callback number presentation indicator.
    pub const CALLBACK_NUM_PRES_IND: u16 = native_tags::CALLBACK_NUM_PRES_IND;
    /// Callback number alphanumeric tag.
    pub const CALLBACK_NUM_ATAG: u16 = native_tags::CALLBACK_NUM_ATAG;
    /// Number of messages.
    pub const NUMBER_OF_MESSAGES: u16 = native_tags::NUMBER_OF_MESSAGES;
    /// Callback number.
    pub const CALLBACK_NUM: u16 = native_tags::CALLBACK_NUM;
    /// DPF result.
    pub const DPF_RESULT: u16 = native_tags::DPF_RESULT;
    /// Set DPF.
    pub const SET_DPF: u16 = native_tags::SET_DPF;
    /// MS availability status.
    pub const MS_AVAILABILITY_STATUS: u16 = native_tags::MS_AVAILABILITY_STATUS;
    /// Network error code.
    pub const NETWORK_ERROR_CODE: u16 = native_tags::NETWORK_ERROR_CODE;
    /// Message payload.
    pub const MESSAGE_PAYLOAD: u16 = native_tags::MESSAGE_PAYLOAD;
    /// Delivery failure reason.
    pub const DELIVERY_FAILURE_REASON: u16 = native_tags::DELIVERY_FAILURE_REASON;
    /// More messages to send.
    pub const MORE_MESSAGES_TO_SEND: u16 = native_tags::MORE_MESSAGES_TO_SEND;
    /// Message state.
    pub const MESSAGE_STATE: u16 = native_tags::MESSAGE_STATE;
    /// Congestion state.
    pub const CONGESTION_STATE: u16 = native_tags::CONGESTION_STATE;
    /// USSD service operation.
    pub const USSD_SERVICE_OP: u16 = native_tags::USSD_SERVICE_OP;
    /// Display time.
    pub const DISPLAY_TIME: u16 = native_tags::DISPLAY_TIME;
    /// SMS signal.
    pub const SMS_SIGNAL: u16 = native_tags::SMS_SIGNAL;
    /// MS validity.
    pub const MS_VALIDITY: u16 = native_tags::MS_VALIDITY;
    /// Alert on message delivery.
    pub const ALERT_ON_MESSAGE_DELIVERY: u16 = native_tags::ALERT_ON_MESSAGE_DELIVERY;
    /// ITS reply type.
    pub const ITS_REPLY_TYPE: u16 = native_tags::ITS_REPLY_TYPE;
    /// ITS session info.
    pub const ITS_SESSION_INFO: u16 = native_tags::ITS_SESSION_INFO;
    /// Broadcast area identifier.
    pub const BROADCAST_AREA_IDENTIFIER: u16 = native_tags::BROADCAST_AREA_IDENTIFIER;
    /// Broadcast content type.
    pub const BROADCAST_CONTENT_TYPE: u16 = native_tags::BROADCAST_CONTENT_TYPE;
    /// Broadcast repetition number.
    pub const BROADCAST_REP_NUM: u16 = native_tags::BROADCAST_REP_NUM;
    /// Broadcast frequency interval.
    pub const BROADCAST_FREQUENCY_INTERVAL: u16 = native_tags::BROADCAST_FREQUENCY_INTERVAL;
    /// Broadcast area success.
    pub const BROADCAST_AREA_SUCCESS: u16 = native_tags::BROADCAST_AREA_SUCCESS;
    /// Broadcast end time.
    pub const BROADCAST_END_TIME: u16 = native_tags::BROADCAST_END_TIME;
    /// Broadcast service group.
    pub const BROADCAST_SERVICE_GROUP: u16 = native_tags::BROADCAST_SERVICE_GROUP;
    /// Broadcast channel indicator.
    pub const BROADCAST_CHANNEL_INDICATOR: u16 = native_tags::BROADCAST_CHANNEL_INDICATOR;
}

/// Gets the tag value by its name.
pub fn get_tag_by_name(name: &str) -> u16 {
    smpp_codec::tlv::get_tag_by_name(name)
}

/// Tag-Length-Value (TLV) Parameter
#[derive(uniffi::Record, Debug, Clone, PartialEq)]
pub struct Tlv {
    /// The tag identifier for the parameter
    pub tag: u16,
    /// The length of the value field
    pub length: u16,
    /// The value of the parameter
    pub value: Vec<u8>,
}

impl From<NativeTlv> for Tlv {
    fn from(value: NativeTlv) -> Self {
        Self {
            tag: value.tag,
            length: value.length,
            value: value.value,
        }
    }
}

impl From<Tlv> for NativeTlv {
    fn from(value: Tlv) -> Self {
        Self {
            tag: value.tag,
            length: value.length,
            value: value.value,
        }
    }
}

/// Creates a new TLV with the given tag and value.
#[uniffi::export]
pub fn tlv_new(tag: u16, value: Vec<u8>) -> Tlv {
    NativeTlv::new(tag, value).into()
}

/// Creates a new TLV with a tag name and value.
#[uniffi::export]
pub fn tlv_new_from_name(name: &str, value: Vec<u8>) -> Tlv {
    NativeTlv::new_from_name(name, value).into()
}

/// Creates a new TLV with a u8 value.
#[uniffi::export]
pub fn tlv_new_u8(tag: u16, value: u8) -> Tlv {
    NativeTlv::new_u8(tag, value).into()
}

/// Creates a new TLV with a tag name and u8 value.
#[uniffi::export]
pub fn tlv_new_u8_from_name(name: &str, value: u8) -> Tlv {
    NativeTlv::new_u8_from_name(name, value).into()
}

/// Creates a new TLV with a u16 value.
#[uniffi::export]
pub fn tlv_new_u16(tag: u16, value: u16) -> Tlv {
    NativeTlv::new_u16(tag, value).into()
}

/// Creates a new TLV with a tag name and u16 value.
#[uniffi::export]
pub fn tlv_new_u16_from_name(name: &str, value: u16) -> Tlv {
    NativeTlv::new_u16_from_name(name, value).into()
}

/// Creates a new TLV with a string value.
#[uniffi::export]
pub fn tlv_new_string(tag: u16, value: String) -> Tlv {
    NativeTlv::new_string(tag, &value).into()
}

/// Creates a new TLV with a message payload.
#[uniffi::export]
pub fn tlv_new_payload(tag: u16, value: Vec<u8>) -> Tlv {
    NativeTlv::new_payload(tag, value).into()
}

/// Encodes a TLV into a byte vector.
///
/// # Panics
///
/// Panics if the internal encoding fails.
#[uniffi::export]
pub fn tlv_encode(tlv: &Tlv) -> Vec<u8> {
    let internal_tlv: NativeTlv = tlv.clone().into();
    let mut buffer = Vec::new();
    // In memory write shouldn't fail
    internal_tlv
        .encode(&mut buffer)
        .expect("Failed to encode TLV");
    buffer
}

use crate::common::SmppFfiError;

/// Decodes a byte buffer into a TLV.
///
/// # Errors
///
/// Returns an error if the decoding fails.
#[uniffi::export]
pub fn tlv_decode(buffer: &[u8]) -> Result<Tlv, SmppFfiError> {
    let mut cursor = std::io::Cursor::new(buffer);
    match NativeTlv::decode(&mut cursor) {
        Ok(Some(native)) => Ok(native.into()),
        Ok(None) => Err(SmppFfiError::Generic {
            msg: "Buffer too short to decode TLV".to_string(),
        }),
        Err(e) => Err(SmppFfiError::Generic {
            msg: format!("Decoding failed: {}", e),
        }),
    }
}

/// Extracts the value of the TLV as a u8.
///
/// # Errors
///
/// Returns an error if the conversion fails.
#[uniffi::export]
pub fn tlv_value_as_u8(tlv: &Tlv) -> Result<u8, SmppFfiError> {
    let internal_tlv: NativeTlv = tlv.clone().into();
    internal_tlv
        .value_as_u8()
        .map_err(|e| SmppFfiError::Generic { msg: e.to_string() })
}

/// Extracts the value of the TLV as a u16.
///
/// # Errors
///
/// Returns an error if the conversion fails.
#[uniffi::export]
pub fn tlv_value_as_u16(tlv: &Tlv) -> Result<u16, SmppFfiError> {
    let internal_tlv: NativeTlv = tlv.clone().into();
    internal_tlv
        .value_as_u16()
        .map_err(|e| SmppFfiError::Generic { msg: e.to_string() })
}

/// Extracts the value of the TLV as a String.
///
/// # Errors
///
/// Returns an error if the conversion fails.
#[uniffi::export]
pub fn tlv_value_as_string(tlv: &Tlv) -> Result<String, SmppFfiError> {
    let internal_tlv: NativeTlv = tlv.clone().into();
    internal_tlv
        .value_as_string()
        .map_err(|e| SmppFfiError::Generic { msg: e.to_string() })
}
