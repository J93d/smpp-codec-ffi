use crate::common::{Npi, SmppFfiError, Ton};
use crate::tlv::Tlv;
use smpp_codec::pdus::{
    DataSm as InternalDataSm, DataSmResp as InternalDataSmResponse,
    DeliverSmRequest as InternalDeliverSmRequest, DeliverSmResponse as InternalDeliverSmResponse,
    DeliveryReceipt as InternalDeliveryReceipt,
};

/// Represents a Data SM request.
#[derive(uniffi::Record, Clone, Debug, PartialEq)]
pub struct DataSm {
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
    /// Type of Number for destination address.
    pub dest_addr_ton: Ton,
    /// Numbering Plan Indicator for destination address.
    pub dest_addr_npi: Npi,
    /// The destination address.
    pub dest_addr: String,
    /// The ESM class.
    pub esm_class: u8,
    /// The registered delivery flag.
    pub registered_delivery: u8,
    /// The data coding scheme.
    pub data_coding: u8,
    /// Optional TLV parameters.
    pub optional_params: Vec<Tlv>,
}

/// Represents a Data SM response.
#[derive(uniffi::Record, Clone, Debug, PartialEq)]
pub struct DataSmResponse {
    /// The sequence number of the PDU.
    pub sequence_number: u32,
    /// The command status.
    pub command_status: u32,
    /// The message ID.
    pub message_id: String,
    /// The status description.
    pub status_description: String,
    /// Optional TLV parameters.
    pub optional_params: Vec<Tlv>,
}

/// Represents a Deliver SM request.
#[derive(uniffi::Record, Clone, Debug, PartialEq)]
pub struct DeliverSmRequest {
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
    /// Type of Number for destination address.
    pub dest_addr_ton: Ton,
    /// Numbering Plan Indicator for destination address.
    pub dest_addr_npi: Npi,
    /// The destination address.
    pub dest_addr: String,
    /// The ESM class.
    pub esm_class: u8,
    /// The protocol ID.
    pub protocol_id: u8,
    /// The priority flag.
    pub priority_flag: u8,
    /// The schedule delivery time.
    pub schedule_delivery_time: Option<String>,
    /// The validity period.
    pub validity_period: Option<String>,
    /// The registered delivery flag.
    pub registered_delivery: u8,
    /// The replace if present flag.
    pub replace_if_present_flag: u8,
    /// The data coding scheme.
    pub data_coding: u8,
    /// The default message ID.
    pub sm_default_msg_id: u8,
    /// The short message content.
    pub short_message: Vec<u8>,
    /// Optional TLV parameters.
    pub optional_params: Vec<Tlv>,
}

/// Represents a Deliver SM response.
#[derive(uniffi::Record, Clone, Debug, PartialEq)]
pub struct DeliverSmResponse {
    /// The sequence number of the PDU.
    pub sequence_number: u32,
    /// The command status.
    pub command_status: u32,
    /// The message ID.
    pub message_id: String,
    /// The status description.
    pub status_description: String,
}

/// Represents a Delivery Receipt.
#[derive(uniffi::Record, Clone, Debug, PartialEq)]
pub struct DeliveryReceipt {
    /// The message ID.
    pub message_id: String,
    /// The number of messages submitted.
    pub submitted_count: u32,
    /// The number of messages delivered.
    pub delivered_count: u32,
    /// The time the message was submitted.
    pub submit_date: String,
    /// The time the message was done.
    pub done_date: String,
    /// The final status.
    pub status: String,
    /// The error code, if any.
    pub error_code: u32,
    /// The text of the receipt.
    pub text: String,
}

/// Encodes a `DataSm` request into a byte vector.
///
/// # Panics
///
/// Panics if the internal encoding fails.
#[uniffi::export]
pub fn encode_data_sm(request: &DataSm) -> Vec<u8> {
    let internal_request = InternalDataSm {
        sequence_number: request.sequence_number,
        service_type: request.service_type.clone(),
        source_addr_ton: request.source_addr_ton.into(),
        source_addr_npi: request.source_addr_npi.into(),
        source_addr: request.source_addr.clone(),
        dest_addr_ton: request.dest_addr_ton.into(),
        dest_addr_npi: request.dest_addr_npi.into(),
        dest_addr: request.dest_addr.clone(),
        esm_class: request.esm_class,
        registered_delivery: request.registered_delivery,
        data_coding: request.data_coding,
        optional_params: request
            .optional_params
            .iter()
            .map(|t| t.clone().into())
            .collect(),
    };
    let mut buffer = Vec::new();
    internal_request
        .encode(&mut buffer)
        .expect("Failed to encode DataSm");
    buffer
}

/// Decodes a byte buffer into a `DataSm` request.
///
/// # Errors
///
/// Returns an error string if the decoding fails.
#[uniffi::export]
pub fn decode_data_sm(buffer: &[u8]) -> Result<DataSm, SmppFfiError> {
    match InternalDataSm::decode(buffer) {
        Ok(internal_request) => Ok(DataSm {
            sequence_number: internal_request.sequence_number,
            service_type: internal_request.service_type,
            source_addr_ton: internal_request.source_addr_ton.into(),
            source_addr_npi: internal_request.source_addr_npi.into(),
            source_addr: internal_request.source_addr,
            dest_addr_ton: internal_request.dest_addr_ton.into(),
            dest_addr_npi: internal_request.dest_addr_npi.into(),
            dest_addr: internal_request.dest_addr,
            esm_class: internal_request.esm_class,
            registered_delivery: internal_request.registered_delivery,
            data_coding: internal_request.data_coding,
            optional_params: internal_request
                .optional_params
                .into_iter()
                .map(|t| t.into())
                .collect(),
        }),
        Err(e) => Err(SmppFfiError::Generic { msg: e.to_string() }),
    }
}

/// Encodes a `DeliverSmRequest` into a byte vector.
///
/// # Panics
///
/// Panics if the internal encoding fails.
#[uniffi::export]
pub fn encode_deliver_sm(request: &DeliverSmRequest) -> Vec<u8> {
    let internal_request = InternalDeliverSmRequest {
        sequence_number: request.sequence_number,
        service_type: request.service_type.clone(),
        source_addr_ton: request.source_addr_ton.into(),
        source_addr_npi: request.source_addr_npi.into(),
        source_addr: request.source_addr.clone(),
        dest_addr_ton: request.dest_addr_ton.into(),
        dest_addr_npi: request.dest_addr_npi.into(),
        dest_addr: request.dest_addr.clone(),
        esm_class: request.esm_class,
        protocol_id: request.protocol_id,
        priority_flag: request.priority_flag,
        schedule_delivery_time: request.schedule_delivery_time.clone().unwrap_or_default(),
        validity_period: request.validity_period.clone().unwrap_or_default(),
        registered_delivery: request.registered_delivery,
        replace_if_present_flag: request.replace_if_present_flag,
        data_coding: request.data_coding,
        sm_default_msg_id: request.sm_default_msg_id,
        short_message: request.short_message.clone(),
        optional_params: request
            .optional_params
            .iter()
            .map(|t| t.clone().into())
            .collect(),
    };
    let mut buffer = Vec::new();
    internal_request
        .encode(&mut buffer)
        .expect("Failed to encode DeliverSm");
    buffer
}

/// Decodes a byte buffer into a `DeliverSmRequest`.
///
/// # Errors
///
/// Returns an error string if the decoding fails.
#[uniffi::export]
pub fn decode_deliver_sm(buffer: &[u8]) -> Result<DeliverSmRequest, SmppFfiError> {
    match InternalDeliverSmRequest::decode(buffer) {
        Ok(internal_request) => Ok(DeliverSmRequest {
            sequence_number: internal_request.sequence_number,
            service_type: internal_request.service_type,
            source_addr_ton: internal_request.source_addr_ton.into(),
            source_addr_npi: internal_request.source_addr_npi.into(),
            source_addr: internal_request.source_addr,
            dest_addr_ton: internal_request.dest_addr_ton.into(),
            dest_addr_npi: internal_request.dest_addr_npi.into(),
            dest_addr: internal_request.dest_addr,
            esm_class: internal_request.esm_class,
            protocol_id: internal_request.protocol_id,
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
            registered_delivery: internal_request.registered_delivery,
            replace_if_present_flag: internal_request.replace_if_present_flag,
            data_coding: internal_request.data_coding,
            sm_default_msg_id: internal_request.sm_default_msg_id,
            short_message: internal_request.short_message,
            optional_params: internal_request
                .optional_params
                .into_iter()
                .map(|t| t.into())
                .collect(),
        }),
        Err(e) => Err(SmppFfiError::Generic { msg: e.to_string() }),
    }
}

/// Encodes a `DeliveryReceipt` into a byte vector (string representation).
#[uniffi::export]
pub fn encode_delivery_receipt(receipt: &DeliveryReceipt) -> Vec<u8> {
    let internal_receipt = InternalDeliveryReceipt {
        message_id: receipt.message_id.clone(),
        submitted_count: receipt.submitted_count,
        delivered_count: receipt.delivered_count,
        submit_date: receipt.submit_date.clone(),
        done_date: receipt.done_date.clone(),
        status: receipt.status.clone(),
        error_code: receipt.error_code,
        text: receipt.text.clone(),
    };
    internal_receipt.to_string().into_bytes()
}

/// Encodes a `DataSmResponse` into a byte vector.
///
/// # Panics
///
/// Panics if the internal encoding fails.
#[uniffi::export]
pub fn encode_data_sm_response(response: &DataSmResponse) -> Vec<u8> {
    let internal_response = InternalDataSmResponse {
        sequence_number: response.sequence_number,
        command_status: response.command_status,
        message_id: response.message_id.clone(),
        status_description: response.status_description.clone(),
        optional_params: response
            .optional_params
            .iter()
            .map(|t| t.clone().into())
            .collect(),
    };
    let mut buffer = Vec::new();
    internal_response
        .encode(&mut buffer)
        .expect("Failed to encode DataSmResponse");
    buffer
}

/// Decodes a byte buffer into a `DataSmResponse`.
///
/// # Errors
///
/// Returns an error string if the decoding fails.
#[uniffi::export]
pub fn decode_data_sm_response(buffer: &[u8]) -> Result<DataSmResponse, SmppFfiError> {
    match InternalDataSmResponse::decode(buffer) {
        Ok(internal_response) => Ok(DataSmResponse {
            sequence_number: internal_response.sequence_number,
            command_status: internal_response.command_status,
            message_id: internal_response.message_id,
            status_description: internal_response.status_description,
            optional_params: internal_response
                .optional_params
                .into_iter()
                .map(|t| t.into())
                .collect(),
        }),
        Err(e) => Err(SmppFfiError::Generic { msg: e.to_string() }),
    }
}

/// Encodes a `DeliverSmResponse` into a byte vector.
///
/// # Panics
///
/// Panics if the internal encoding fails.
#[uniffi::export]
pub fn encode_delivery_sm_response(response: &DeliverSmResponse) -> Vec<u8> {
    let internal_response = InternalDeliverSmResponse {
        sequence_number: response.sequence_number,
        command_status: response.command_status,
        message_id: response.message_id.clone(),
        status_description: response.status_description.clone(),
    };
    let mut buffer = Vec::new();
    internal_response
        .encode(&mut buffer)
        .expect("Failed to encode DeliverSmResponse");
    buffer
}

/// Decodes a byte buffer into a `DeliverSmResponse`.
///
/// # Errors
///
/// Returns an error string if the decoding fails.
#[uniffi::export]
pub fn decode_delivery_sm_response(buffer: &[u8]) -> Result<DeliverSmResponse, SmppFfiError> {
    match InternalDeliverSmResponse::decode(buffer) {
        Ok(internal_response) => Ok(DeliverSmResponse {
            sequence_number: internal_response.sequence_number,
            command_status: internal_response.command_status,
            message_id: internal_response.message_id,
            status_description: internal_response.status_description,
        }),
        Err(e) => Err(SmppFfiError::Generic { msg: e.to_string() }),
    }
}
