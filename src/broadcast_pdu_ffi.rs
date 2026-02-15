use crate::common::{Npi, Ton};
use crate::tlv::Tlv;
use smpp_codec::pdus::{
    BroadcastSm as NativeBroadcastSmRequest, BroadcastSmResp as NativeBroadcastSmResponse,
};

/// Represents a Broadcast SM request.
#[derive(uniffi::Record, Clone, Debug, PartialEq)]
pub struct BroadcastSmRequest {
    /// The sequence number of the PDU.
    pub sequence_number: u32,
    /// The service type.
    pub service_type: String,
    /// Type of Number for source address.
    pub source_addr_ton: Ton,
    /// Numbering Plan Indicator for source address.
    pub source_addr_npi: Npi,
    /// The source address.
    pub source_addr: String,
    /// The message ID.
    pub message_id: String,
    /// The priority flag.
    pub priority_flag: u8,
    /// The schedule delivery time.
    pub schedule_delivery_time: Option<String>,
    /// The validity period.
    pub validity_period: Option<String>,
    /// The replace if present flag.
    pub replace_if_present_flag: u8,
    /// The data coding scheme.
    pub data_coding: u8,
    /// The default message ID.
    pub sm_default_msg_id: u8,
    /// Optional TLV parameters.
    pub optional_params: Vec<Tlv>,
}

/// Represents a Broadcast SM response.
#[derive(uniffi::Record, Clone, Debug, PartialEq)]
pub struct BroadcastSmResponse {
    /// The sequence number of the PDU.
    pub sequence_number: u32,
    /// The command status.
    pub command_status: u32,
    /// The status description.
    pub status_description: String,
    /// The message ID.
    pub message_id: String,
    /// Optional TLV parameters.
    pub optional_params: Vec<Tlv>,
}

/// Encodes a `BroadcastSmRequest` into a byte vector.
///
/// # Panics
///
/// Panics if the internal encoding fails.
#[uniffi::export]
pub fn encode_broadcast_sm_request(request: &BroadcastSmRequest) -> Vec<u8> {
    let internal_request = NativeBroadcastSmRequest {
        sequence_number: request.sequence_number,
        service_type: request.service_type.clone(),
        source_addr_ton: request.source_addr_ton.into(),
        source_addr_npi: request.source_addr_npi.into(),
        source_addr: request.source_addr.clone(),
        message_id: request.message_id.clone(),
        priority_flag: request.priority_flag,
        schedule_delivery_time: request.schedule_delivery_time.clone().unwrap_or_default(),
        validity_period: request.validity_period.clone().unwrap_or_default(),
        replace_if_present_flag: request.replace_if_present_flag,
        data_coding: request.data_coding,
        sm_default_msg_id: request.sm_default_msg_id,
        optional_params: request
            .optional_params
            .iter()
            .map(|t| t.clone().into())
            .collect(),
    };
    let mut buffer = Vec::new();
    internal_request
        .encode(&mut buffer)
        .expect("Failed to encode BroadcastSmRequest");
    buffer
}

/// Decodes a byte buffer into a `BroadcastSmRequest`.
///
/// # Errors
///
/// Returns an error string if the decoding fails.
#[uniffi::export]
pub fn decode_broadcast_sm_request(buffer: &[u8]) -> Result<BroadcastSmRequest, String> {
    match NativeBroadcastSmRequest::decode(buffer) {
        Ok(internal_request) => Ok(BroadcastSmRequest {
            sequence_number: internal_request.sequence_number,
            service_type: internal_request.service_type,
            source_addr_ton: internal_request.source_addr_ton.into(),
            source_addr_npi: internal_request.source_addr_npi.into(),
            source_addr: internal_request.source_addr,
            message_id: internal_request.message_id,
            priority_flag: internal_request.priority_flag,
            schedule_delivery_time: if internal_request.schedule_delivery_time.is_empty() {
                None
            } else {
                Some(internal_request.schedule_delivery_time)
            },
            validity_period: if internal_request.validity_period.is_empty() {
                None
            } else {
                Some(internal_request.validity_period)
            },
            replace_if_present_flag: internal_request.replace_if_present_flag,
            data_coding: internal_request.data_coding,
            sm_default_msg_id: internal_request.sm_default_msg_id,
            optional_params: internal_request
                .optional_params
                .into_iter()
                .map(|t| t.into())
                .collect(),
        }),
        Err(e) => Err(e.to_string()),
    }
}

/// Encodes a `BroadcastSmResponse` into a byte vector.
///
/// # Panics
///
/// Panics if the internal encoding fails.
#[uniffi::export]
pub fn encode_broadcast_sm_response(response: &BroadcastSmResponse) -> Vec<u8> {
    let internal_response = NativeBroadcastSmResponse {
        sequence_number: response.sequence_number,
        command_status: response.command_status,
        status_description: response.status_description.clone(),
        message_id: response.message_id.clone(),
        optional_params: response
            .optional_params
            .iter()
            .map(|t| t.clone().into())
            .collect(),
    };
    let mut buffer = Vec::new();
    internal_response
        .encode(&mut buffer)
        .expect("Failed to encode BroadcastSmResponse");
    buffer
}

/// Decodes a byte buffer into a `BroadcastSmResponse`.
///
/// # Errors
///
/// Returns an error string if the decoding fails.
#[uniffi::export]
pub fn decode_broadcast_sm_response(buffer: &[u8]) -> Result<BroadcastSmResponse, String> {
    match NativeBroadcastSmResponse::decode(buffer) {
        Ok(internal_response) => Ok(BroadcastSmResponse {
            sequence_number: internal_response.sequence_number,
            command_status: internal_response.command_status,
            status_description: internal_response.status_description,
            message_id: internal_response.message_id,
            optional_params: internal_response
                .optional_params
                .into_iter()
                .map(|t| t.into())
                .collect(),
        }),
        Err(e) => Err(e.to_string()),
    }
}
