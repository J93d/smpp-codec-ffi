use crate::common::{Npi, SmppFfiError, Ton};
use crate::tlv::Tlv;
use smpp_codec::pdus::{
    CancelBroadcastSm as NativeCancelBroadcastSmRequest,
    CancelBroadcastSmResp as NativeCancelBroadcastSmResponse,
    CancelSmRequest as NativeCancelSmRequest, CancelSmResponse as NativeCancelSmResponse,
    MessageState as NativeMessageState, QueryBroadcastSm as NativeQueryBroadcastSmRequest,
    QueryBroadcastSmResp as NativeQueryBroadcastSmResponse, QuerySmRequest as NativeQuerySmRequest,
    QuerySmResponse as NativeQuerySmResponse, ReplaceSm as NativeReplaceSmRequest,
    ReplaceSmResp as NativeReplaceSmResponse,
};

/// Represents the state of a message.
#[derive(uniffi::Enum, Clone, Debug, PartialEq)]
pub enum MessageState {
    /// Message is in enroute state
    Enroute,
    /// Message is delivered
    Delivered,
    /// Message validity period has expired
    Expired,
    /// Message has been deleted
    Deleted,
    /// Message is undeliverable
    Undeliverable,
    /// Message is in accepted state
    Accepted,
    /// Message is in invalid state
    Unknown,
    /// Message is in rejected state
    Rejected,
}

impl From<NativeMessageState> for MessageState {
    fn from(state: NativeMessageState) -> Self {
        match state {
            NativeMessageState::Enroute => MessageState::Enroute,
            NativeMessageState::Delivered => MessageState::Delivered,
            NativeMessageState::Expired => MessageState::Expired,
            NativeMessageState::Deleted => MessageState::Deleted,
            NativeMessageState::Undeliverable => MessageState::Undeliverable,
            NativeMessageState::Accepted => MessageState::Accepted,
            NativeMessageState::Unknown => MessageState::Unknown,
            NativeMessageState::Rejected => MessageState::Rejected,
        }
    }
}

impl From<MessageState> for NativeMessageState {
    fn from(state: MessageState) -> Self {
        match state {
            MessageState::Enroute => NativeMessageState::Enroute,
            MessageState::Delivered => NativeMessageState::Delivered,
            MessageState::Expired => NativeMessageState::Expired,
            MessageState::Deleted => NativeMessageState::Deleted,
            MessageState::Undeliverable => NativeMessageState::Undeliverable,
            MessageState::Accepted => NativeMessageState::Accepted,
            MessageState::Unknown => NativeMessageState::Unknown,
            MessageState::Rejected => NativeMessageState::Rejected,
        }
    }
}

/// Represents a Cancel Broadcast SM request.
#[derive(uniffi::Record, Clone, Debug, PartialEq)]
pub struct CancelBroadcastSmRequest {
    /// The sequence number of the PDU.
    pub sequence_number: u32,
    /// The service type.
    pub service_type: String,
    /// The message ID.
    pub message_id: String,
    /// Type of Number for source address.
    pub source_addr_ton: Ton,
    /// Numbering Plan Indicator for source address.
    pub source_addr_npi: Npi,
    /// The source address.
    pub source_addr: String,
    /// Optional TLV parameters.
    pub optional_params: Vec<Tlv>,
}

/// Represents a Cancel Broadcast SM response.
#[derive(uniffi::Record, Clone, Debug, PartialEq)]
pub struct CancelBroadcastSmResponse {
    /// The sequence number of the PDU.
    pub sequence_number: u32,
    /// The command status.
    pub command_status: u32,
    /// The status description.
    pub status_description: String,
}

/// Represents a Cancel SM request.
#[derive(uniffi::Record, Clone, Debug, PartialEq)]
pub struct CancelSmRequest {
    /// The sequence number of the PDU.
    pub sequence_number: u32,
    /// The service type.
    pub service_type: String,
    /// The message ID.
    pub message_id: String,
    /// Type of Number for source address.
    pub source_addr_ton: Ton,
    /// Numbering Plan Indicator for source address.
    pub source_addr_npi: Npi,
    /// The source address.
    pub source_addr: String,
    /// Type of Number for destination address.
    pub dest_addr_ton: Ton,
    /// Numbering Plan Indicator for destination address.
    pub dest_addr_npi: Npi,
    /// The destination address.
    pub dest_addr: String,
}

/// Represents a Cancel SM response.
#[derive(uniffi::Record, Clone, Debug, PartialEq)]
pub struct CancelSmResponse {
    /// The sequence number of the PDU.
    pub sequence_number: u32,
    /// The command status.
    pub command_status: u32,
    /// The status description.
    pub status_description: String,
}

/// Represents a Query SM request.
#[derive(uniffi::Record, Clone, Debug, PartialEq)]
pub struct QuerySmRequest {
    /// The sequence number of the PDU.
    pub sequence_number: u32,
    /// The message ID.
    pub message_id: String,
    /// Type of Number for source address.
    pub source_addr_ton: Ton,
    /// Numbering Plan Indicator for source address.
    pub source_addr_npi: Npi,
    /// The source address.
    pub source_addr: String,
}

/// Represents a Query SM response.
#[derive(uniffi::Record, Clone, Debug, PartialEq)]
pub struct QuerySmResponse {
    /// The sequence number of the PDU.
    pub sequence_number: u32,
    /// The command status.
    pub command_status: u32,
    /// The message ID.
    pub message_id: String,
    /// The final date.
    pub final_date: String,
    /// The message state.
    pub message_state: u8,
    /// The error code.
    pub error_code: u8,
    /// The status description.
    pub status_description: String,
}

/// Represents a Query Broadcast SM request.
#[derive(uniffi::Record, Clone, Debug, PartialEq)]
pub struct QueryBroadcastSmRequest {
    /// The sequence number of the PDU.
    pub sequence_number: u32,
    /// The message ID.
    pub message_id: String,
    /// Type of Number for source address.
    pub source_addr_ton: Ton,
    /// Numbering Plan Indicator for source address.
    pub source_addr_npi: Npi,
    /// The source address.
    pub source_addr: String,
    /// Optional TLV parameters.
    pub optional_params: Vec<Tlv>,
}

/// Represents a Query Broadcast SM response.
#[derive(uniffi::Record, Clone, Debug, PartialEq)]
pub struct QueryBroadcastSmResponse {
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

/// Represents a Replace SM request.
#[derive(uniffi::Record, Clone, Debug, PartialEq)]
pub struct ReplaceSmRequest {
    /// The sequence number of the PDU.
    pub sequence_number: u32,
    /// The message ID.
    pub message_id: String,
    /// Type of Number for source address.
    pub source_addr_ton: Ton,
    /// Numbering Plan Indicator for source address.
    pub source_addr_npi: Npi,
    /// The source address.
    pub source_addr: String,
    /// The schedule delivery time.
    pub schedule_delivery_time: String,
    /// The validity period.
    pub validity_period: String,
    /// The registered delivery flag.
    pub registered_delivery: u8,
    /// The default message ID.
    pub sm_default_msg_id: u8,
    /// The short message content.
    pub short_message: Vec<u8>,
}

/// Represents a Replace SM response.
#[derive(uniffi::Record, Clone, Debug, PartialEq)]
pub struct ReplaceSmResponse {
    /// The sequence number of the PDU.
    pub sequence_number: u32,
    /// The command status.
    pub command_status: u32,
    /// The status description.
    pub status_description: String,
}

/// Encodes a `CancelBroadcastSmRequest` into a byte vector.
///
/// # Panics
///
/// Panics if the internal encoding fails.
#[uniffi::export]
pub fn encode_cancel_broadcast_sm_request(request: &CancelBroadcastSmRequest) -> Vec<u8> {
    let internal_request = NativeCancelBroadcastSmRequest {
        sequence_number: request.sequence_number,
        service_type: request.service_type.clone(),
        message_id: request.message_id.clone(),
        source_addr_ton: request.source_addr_ton.into(),
        source_addr_npi: request.source_addr_npi.into(),
        source_addr: request.source_addr.clone(),
        optional_params: request
            .optional_params
            .iter()
            .map(|t| t.clone().into())
            .collect(),
    };
    let mut buffer = Vec::new();
    internal_request
        .encode(&mut buffer)
        .expect("Failed to encode CancelBroadcastSmRequest");
    buffer
}

/// Decodes a byte buffer into a `CancelBroadcastSmRequest`.
///
/// # Errors
///
/// Returns an error string if the decoding fails.
#[uniffi::export]
pub fn decode_cancel_broadcast_sm_request(
    buffer: &[u8],
) -> Result<CancelBroadcastSmRequest, SmppFfiError> {
    match NativeCancelBroadcastSmRequest::decode(buffer) {
        Ok(internal_request) => Ok(CancelBroadcastSmRequest {
            sequence_number: internal_request.sequence_number,
            service_type: internal_request.service_type,
            message_id: internal_request.message_id,
            source_addr_ton: internal_request.source_addr_ton.into(),
            source_addr_npi: internal_request.source_addr_npi.into(),
            source_addr: internal_request.source_addr,
            optional_params: internal_request
                .optional_params
                .iter()
                .map(|t| t.clone().into())
                .collect(),
        }),
        Err(e) => Err(SmppFfiError::Generic { msg: e.to_string() }),
    }
}

/// Encodes a `CancelBroadcastSmResponse` into a byte vector.
///
/// # Panics
///
/// Panics if the internal encoding fails.
#[uniffi::export]
pub fn encode_cancel_broadcast_sm_response(response: &CancelBroadcastSmResponse) -> Vec<u8> {
    let internal_response = NativeCancelBroadcastSmResponse {
        sequence_number: response.sequence_number,
        command_status: response.command_status,
        status_description: response.status_description.clone(),
    };
    let mut buffer = Vec::new();
    internal_response
        .encode(&mut buffer)
        .expect("Failed to encode CancelBroadcastSmResponse");
    buffer
}

/// Decodes a byte buffer into a `CancelBroadcastSmResponse`.
///
/// # Errors
///
/// Returns an error string if the decoding fails.
#[uniffi::export]
pub fn decode_cancel_broadcast_sm_response(
    buffer: &[u8],
) -> Result<CancelBroadcastSmResponse, SmppFfiError> {
    match NativeCancelBroadcastSmResponse::decode(buffer) {
        Ok(internal_response) => Ok(CancelBroadcastSmResponse {
            sequence_number: internal_response.sequence_number,
            command_status: internal_response.command_status,
            status_description: internal_response.status_description,
        }),
        Err(e) => Err(SmppFfiError::Generic { msg: e.to_string() }),
    }
}

/// Encodes a `CancelSmRequest` into a byte vector.
///
/// # Panics
///
/// Panics if the internal encoding fails.
#[uniffi::export]
pub fn encode_cancel_sm_request(request: &CancelSmRequest) -> Vec<u8> {
    let internal_request = NativeCancelSmRequest {
        sequence_number: request.sequence_number,
        service_type: request.service_type.clone(),
        message_id: request.message_id.clone(),
        source_addr_ton: request.source_addr_ton.into(),
        source_addr_npi: request.source_addr_npi.into(),
        source_addr: request.source_addr.clone(),
        dest_addr_ton: request.dest_addr_ton.into(),
        dest_addr_npi: request.dest_addr_npi.into(),
        dest_addr: request.dest_addr.clone(),
    };
    let mut buffer = Vec::new();
    internal_request
        .encode(&mut buffer)
        .expect("Failed to encode CancelSmRequest");
    buffer
}

/// Decodes a byte buffer into a `CancelSmRequest`.
///
/// # Errors
///
/// Returns an error string if the decoding fails.
#[uniffi::export]
pub fn decode_cancel_sm_request(buffer: &[u8]) -> Result<CancelSmRequest, SmppFfiError> {
    match NativeCancelSmRequest::decode(buffer) {
        Ok(internal_request) => Ok(CancelSmRequest {
            sequence_number: internal_request.sequence_number,
            service_type: internal_request.service_type,
            message_id: internal_request.message_id,
            source_addr_ton: internal_request.source_addr_ton.into(),
            source_addr_npi: internal_request.source_addr_npi.into(),
            source_addr: internal_request.source_addr,
            dest_addr_ton: internal_request.dest_addr_ton.into(),
            dest_addr_npi: internal_request.dest_addr_npi.into(),
            dest_addr: internal_request.dest_addr,
        }),
        Err(e) => Err(SmppFfiError::Generic { msg: e.to_string() }),
    }
}

/// Encodes a `CancelSmResponse` into a byte vector.
///
/// # Panics
///
/// Panics if the internal encoding fails.
#[uniffi::export]
pub fn encode_cancel_sm_response(response: &CancelSmResponse) -> Vec<u8> {
    let internal_response = NativeCancelSmResponse {
        sequence_number: response.sequence_number,
        command_status: response.command_status,
        status_description: response.status_description.clone(),
    };
    let mut buffer = Vec::new();
    internal_response
        .encode(&mut buffer)
        .expect("Failed to encode CancelSmResponse");
    buffer
}

/// Decodes a byte buffer into a `CancelSmResponse`.
///
/// # Errors
///
/// Returns an error string if the decoding fails.
#[uniffi::export]
pub fn decode_cancel_sm_response(buffer: &[u8]) -> Result<CancelSmResponse, SmppFfiError> {
    match NativeCancelSmResponse::decode(buffer) {
        Ok(internal_response) => Ok(CancelSmResponse {
            sequence_number: internal_response.sequence_number,
            command_status: internal_response.command_status,
            status_description: internal_response.status_description,
        }),
        Err(e) => Err(SmppFfiError::Generic { msg: e.to_string() }),
    }
}

/// Encodes a `QuerySmRequest` into a byte vector.
///
/// # Panics
///
/// Panics if the internal encoding fails.
#[uniffi::export]
pub fn encode_query_sm_request(request: &QuerySmRequest) -> Vec<u8> {
    let internal_request = NativeQuerySmRequest {
        sequence_number: request.sequence_number,
        message_id: request.message_id.clone(),
        source_addr_ton: request.source_addr_ton.into(),
        source_addr_npi: request.source_addr_npi.into(),
        source_addr: request.source_addr.clone(),
    };
    let mut buffer = Vec::new();
    internal_request
        .encode(&mut buffer)
        .expect("Failed to encode QuerySmRequest");
    buffer
}

/// Decodes a byte buffer into a `QuerySmRequest`.
///
/// # Errors
///
/// Returns an error string if the decoding fails.
#[uniffi::export]
pub fn decode_query_sm_request(buffer: &[u8]) -> Result<QuerySmRequest, SmppFfiError> {
    match NativeQuerySmRequest::decode(buffer) {
        Ok(internal_request) => Ok(QuerySmRequest {
            sequence_number: internal_request.sequence_number,
            message_id: internal_request.message_id,
            source_addr_ton: internal_request.source_addr_ton.into(),
            source_addr_npi: internal_request.source_addr_npi.into(),
            source_addr: internal_request.source_addr,
        }),
        Err(e) => Err(SmppFfiError::Generic { msg: e.to_string() }),
    }
}

/// Encodes a `QuerySmResponse` into a byte vector.
///
/// # Panics
///
/// Panics if the internal encoding fails.
#[uniffi::export]
pub fn encode_query_sm_response(response: &QuerySmResponse) -> Vec<u8> {
    let internal_response = NativeQuerySmResponse {
        sequence_number: response.sequence_number,
        command_status: response.command_status,
        message_id: response.message_id.clone(),
        final_date: response.final_date.clone(),
        message_state: response.message_state,
        error_code: response.error_code,
        status_description: response.status_description.clone(),
    };
    let mut buffer = Vec::new();
    internal_response
        .encode(&mut buffer)
        .expect("Failed to encode QuerySmResponse");
    buffer
}

/// Decodes a byte buffer into a `QuerySmResponse`.
///
/// # Errors
///
/// Returns an error string if the decoding fails.
#[uniffi::export]
pub fn decode_query_sm_response(buffer: &[u8]) -> Result<QuerySmResponse, SmppFfiError> {
    match NativeQuerySmResponse::decode(buffer) {
        Ok(internal_response) => Ok(QuerySmResponse {
            sequence_number: internal_response.sequence_number,
            command_status: internal_response.command_status,
            message_id: internal_response.message_id,
            final_date: internal_response.final_date,
            message_state: internal_response.message_state,
            error_code: internal_response.error_code,
            status_description: internal_response.status_description,
        }),
        Err(e) => Err(SmppFfiError::Generic { msg: e.to_string() }),
    }
}

/// Encodes a `QueryBroadcastSmRequest` into a byte vector.
///
/// # Panics
///
/// Panics if the internal encoding fails.
#[uniffi::export]
pub fn encode_query_broadcast_sm_request(request: &QueryBroadcastSmRequest) -> Vec<u8> {
    let internal_request = NativeQueryBroadcastSmRequest {
        sequence_number: request.sequence_number,
        message_id: request.message_id.clone(),
        source_addr_ton: request.source_addr_ton.into(),
        source_addr_npi: request.source_addr_npi.into(),
        source_addr: request.source_addr.clone(),
        optional_params: request
            .optional_params
            .iter()
            .map(|t| t.clone().into())
            .collect(),
    };
    let mut buffer = Vec::new();
    internal_request
        .encode(&mut buffer)
        .expect("Failed to encode QueryBroadcastSmRequest");
    buffer
}

/// Decodes a byte buffer into a `QueryBroadcastSmRequest`.
///
/// # Errors
///
/// Returns an error string if the decoding fails.
#[uniffi::export]
pub fn decode_query_broadcast_sm_request(
    buffer: &[u8],
) -> Result<QueryBroadcastSmRequest, SmppFfiError> {
    match NativeQueryBroadcastSmRequest::decode(buffer) {
        Ok(internal_request) => Ok(QueryBroadcastSmRequest {
            sequence_number: internal_request.sequence_number,
            message_id: internal_request.message_id,
            source_addr_ton: internal_request.source_addr_ton.into(),
            source_addr_npi: internal_request.source_addr_npi.into(),
            source_addr: internal_request.source_addr,
            optional_params: internal_request
                .optional_params
                .into_iter()
                .map(|t| t.into())
                .collect(),
        }),
        Err(e) => Err(SmppFfiError::Generic { msg: e.to_string() }),
    }
}

/// Encodes a `QueryBroadcastSmResponse` into a byte vector.
///
/// # Panics
///
/// Panics if the internal encoding fails.
#[uniffi::export]
pub fn encode_query_broadcast_sm_response(response: &QueryBroadcastSmResponse) -> Vec<u8> {
    let internal_response = NativeQueryBroadcastSmResponse {
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
        .expect("Failed to encode QueryBroadcastSmResponse");
    buffer
}

/// Decodes a byte buffer into a `QueryBroadcastSmResponse`.
///
/// # Errors
///
/// Returns an error string if the decoding fails.
#[uniffi::export]
pub fn decode_query_broadcast_sm_response(
    buffer: &[u8],
) -> Result<QueryBroadcastSmResponse, SmppFfiError> {
    match NativeQueryBroadcastSmResponse::decode(buffer) {
        Ok(internal_response) => Ok(QueryBroadcastSmResponse {
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
        Err(e) => Err(SmppFfiError::Generic { msg: e.to_string() }),
    }
}

/// Encodes a `ReplaceSmRequest` into a byte vector.
///
/// # Panics
///
/// Panics if the internal encoding fails.
#[uniffi::export]
pub fn encode_replace_sm_request(request: &ReplaceSmRequest) -> Vec<u8> {
    let internal_request = NativeReplaceSmRequest {
        sequence_number: request.sequence_number,
        message_id: request.message_id.clone(),
        source_addr_ton: request.source_addr_ton.into(),
        source_addr_npi: request.source_addr_npi.into(),
        source_addr: request.source_addr.clone(),
        schedule_delivery_time: request.schedule_delivery_time.clone(),
        validity_period: request.validity_period.clone(),
        registered_delivery: request.registered_delivery,
        sm_default_msg_id: request.sm_default_msg_id,
        short_message: request.short_message.clone(),
    };
    let mut buffer = Vec::new();
    internal_request
        .encode(&mut buffer)
        .expect("Failed to encode ReplaceSmRequest");
    buffer
}

/// Decodes a byte buffer into a `ReplaceSmRequest`.
///
/// # Errors
///
/// Returns an error string if the decoding fails.
#[uniffi::export]
pub fn decode_replace_sm_request(buffer: &[u8]) -> Result<ReplaceSmRequest, SmppFfiError> {
    match NativeReplaceSmRequest::decode(buffer) {
        Ok(internal_request) => Ok(ReplaceSmRequest {
            sequence_number: internal_request.sequence_number,
            message_id: internal_request.message_id,
            source_addr_ton: internal_request.source_addr_ton.into(),
            source_addr_npi: internal_request.source_addr_npi.into(),
            source_addr: internal_request.source_addr,
            schedule_delivery_time: internal_request.schedule_delivery_time,
            validity_period: internal_request.validity_period,
            registered_delivery: internal_request.registered_delivery,
            sm_default_msg_id: internal_request.sm_default_msg_id,
            short_message: internal_request.short_message,
        }),
        Err(e) => Err(SmppFfiError::Generic { msg: e.to_string() }),
    }
}

/// Encodes a `ReplaceSmResponse` into a byte vector.
///
/// # Panics
///
/// Panics if the internal encoding fails.
#[uniffi::export]
pub fn encode_replace_sm_response(response: &ReplaceSmResponse) -> Vec<u8> {
    let internal_response = NativeReplaceSmResponse {
        sequence_number: response.sequence_number,
        command_status: response.command_status,
        status_description: response.status_description.clone(),
    };
    let mut buffer = Vec::new();
    internal_response
        .encode(&mut buffer)
        .expect("Failed to encode ReplaceSmResponse");
    buffer
}

/// Decodes a byte buffer into a `ReplaceSmResponse`.
///
/// # Errors
///
/// Returns an error string if the decoding fails.
#[uniffi::export]
pub fn decode_replace_sm_response(buffer: &[u8]) -> Result<ReplaceSmResponse, SmppFfiError> {
    match NativeReplaceSmResponse::decode(buffer) {
        Ok(internal_response) => Ok(ReplaceSmResponse {
            sequence_number: internal_response.sequence_number,
            command_status: internal_response.command_status,
            status_description: internal_response.status_description,
        }),
        Err(e) => Err(SmppFfiError::Generic { msg: e.to_string() }),
    }
}
