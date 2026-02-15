use crate::common::{Npi, Ton};
use crate::tlv::Tlv;
use smpp_codec::pdus::{
    Destination as NativeDestination, SubmitMulti as InternalSubmitMultiRequest,
    SubmitMultiResp as InternalSubmitMultiResponse, SubmitSmRequest as InternalSubmitSmRequest,
    SubmitSmResponse as InternalSubmitSmResponse,
    UnsuccessfulDelivery as NativeUnsuccessfulDelivery,
};

/// Represents a Submit SM request.
#[derive(uniffi::Record, Clone, Debug, PartialEq)]
pub struct SubmitSmRequest {
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
    pub destination_addr: String,
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
    pub tlvs: Vec<Tlv>,
}

/// Represents a Submit SM response.
#[derive(uniffi::Record, Clone, Debug, PartialEq)]
pub struct SubmitSmResponse {
    /// The sequence number of the PDU.
    pub sequence_number: u32,
    /// The command status.
    pub command_status: u32,
    /// The message ID.
    pub message_id: String,
    /// The status description.
    pub status_description: String,
}

/// Represents a destination for Submit Multi.
#[derive(uniffi::Enum, Clone, Debug, PartialEq)]
pub enum Destination {
    /// SME Address
    SmeAddress {
        /// Type of Number.
        ton: Ton,
        /// Numbering Plan Indicator.
        npi: Npi,
        /// The address.
        address: String,
    },
    /// Distribution List
    DistributionList {
        /// The name of the distribution list.
        name: String,
    },
}

impl From<NativeDestination> for Destination {
    fn from(native: NativeDestination) -> Self {
        match native {
            NativeDestination::SmeAddress { ton, npi, address } => Destination::SmeAddress {
                ton: ton.into(),
                npi: npi.into(),
                address,
            },
            NativeDestination::DistributionList(name) => Destination::DistributionList { name },
        }
    }
}

impl From<Destination> for NativeDestination {
    fn from(ffi: Destination) -> Self {
        match ffi {
            Destination::SmeAddress { ton, npi, address } => NativeDestination::SmeAddress {
                ton: ton.into(),
                npi: npi.into(),
                address,
            },
            Destination::DistributionList { name } => NativeDestination::DistributionList(name),
        }
    }
}

/// Represents a Submit Multi request.
#[derive(uniffi::Record, Clone, Debug, PartialEq)]
pub struct SubmitMultiRequest {
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
    /// List of destinations (SME addresses or Distribution Lists).
    pub destinations: Vec<Destination>,
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
    pub tlvs: Vec<Tlv>,
}

/// Represents an unsuccessful delivery attempt in Submit Multi response.
#[derive(uniffi::Record, Clone, Debug, PartialEq)]
pub struct UnsuccessfulDelivery {
    /// Type of Number for the address.
    pub ton: Ton,
    /// Numbering Plan Indicator for the address.
    pub npi: Npi,
    /// The address that failed.
    pub address: String,
    /// The error status code.
    pub error_status: u32,
}

impl From<NativeUnsuccessfulDelivery> for UnsuccessfulDelivery {
    fn from(native: NativeUnsuccessfulDelivery) -> Self {
        Self {
            ton: native.ton.into(),
            npi: native.npi.into(),
            address: native.address,
            error_status: native.error_status,
        }
    }
}

impl From<UnsuccessfulDelivery> for NativeUnsuccessfulDelivery {
    fn from(ffi: UnsuccessfulDelivery) -> Self {
        Self {
            ton: ffi.ton.into(),
            npi: ffi.npi.into(),
            address: ffi.address,
            error_status: ffi.error_status,
        }
    }
}

/// Represents a Submit Multi response.
#[derive(uniffi::Record, Clone, Debug, PartialEq)]
pub struct SubmitMultiResponse {
    /// The sequence number of the PDU.
    pub sequence_number: u32,
    /// The command status.
    pub command_status: u32,
    /// The message ID.
    pub message_id: String,
    /// The status description.
    pub status_description: String,
    /// List of unsuccessful deliveries.
    pub unsuccess_smes: Vec<UnsuccessfulDelivery>,
}

/// Encodes a `SubmitSmRequest` into a byte vector.
///
/// # Panics
///
/// Panics if the internal encoding fails (which should not happen for valid input).
#[uniffi::export]
pub fn encode_submit_sm_request(request: &SubmitSmRequest) -> Vec<u8> {
    let internal_request = InternalSubmitSmRequest {
        sequence_number: request.sequence_number,
        service_type: request.service_type.clone(),
        source_addr_ton: request.source_addr_ton.into(),
        source_addr_npi: request.source_addr_npi.into(),
        source_addr: request.source_addr.clone(),
        dest_addr_ton: request.dest_addr_ton.into(),
        dest_addr_npi: request.dest_addr_npi.into(),
        dest_addr: request.destination_addr.clone(),
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
        optional_params: request.tlvs.iter().map(|t| t.clone().into()).collect(),
    };
    let mut buffer = Vec::new();
    internal_request
        .encode(&mut buffer)
        .expect("Failed to encode SubmitSmRequest");
    buffer
}

/// Decodes a byte buffer into a `SubmitSmRequest`.
///
/// # Errors
///
/// Returns an error string if the decoding fails.
#[uniffi::export]
pub fn decode_submit_sm_request(buffer: &[u8]) -> Result<SubmitSmRequest, String> {
    match InternalSubmitSmRequest::decode(buffer) {
        Ok(internal_request) => Ok(SubmitSmRequest {
            sequence_number: internal_request.sequence_number,
            service_type: internal_request.service_type,
            source_addr_ton: internal_request.source_addr_ton.into(),
            source_addr_npi: internal_request.source_addr_npi.into(),
            source_addr: internal_request.source_addr,
            dest_addr_ton: internal_request.dest_addr_ton.into(),
            dest_addr_npi: internal_request.dest_addr_npi.into(),
            destination_addr: internal_request.dest_addr,
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
            tlvs: internal_request
                .optional_params
                .into_iter()
                .map(|t| t.into())
                .collect(),
        }),
        Err(e) => Err(e.to_string()),
    }
}

/// Encodes a `SubmitSmResponse` into a byte vector.
///
/// # Panics
///
/// Panics if the internal encoding fails.
#[uniffi::export]
pub fn encode_submit_sm_response(response: &SubmitSmResponse) -> Vec<u8> {
    let internal_response = InternalSubmitSmResponse {
        sequence_number: response.sequence_number,
        command_status: response.command_status,
        message_id: response.message_id.clone(),
        status_description: response.status_description.clone(),
    };
    let mut buffer = Vec::new();
    internal_response
        .encode(&mut buffer)
        .expect("Failed to encode SubmitSmResponse");
    buffer
}

/// Decodes a byte buffer into a `SubmitSmResponse`.
///
/// # Errors
///
/// Returns an error string if the decoding fails.
#[uniffi::export]
pub fn decode_submit_sm_response(buffer: &[u8]) -> Result<SubmitSmResponse, String> {
    match InternalSubmitSmResponse::decode(buffer) {
        Ok(internal_resp) => Ok(SubmitSmResponse {
            sequence_number: internal_resp.sequence_number,
            command_status: internal_resp.command_status,
            message_id: internal_resp.message_id,
            status_description: internal_resp.status_description,
        }),
        Err(e) => Err(e.to_string()),
    }
}

/// Encodes a `SubmitMultiRequest` into a byte vector.
///
/// # Panics
///
/// Panics if the internal encoding fails.
#[uniffi::export]
pub fn encode_submit_multi_request(request: &SubmitMultiRequest) -> Vec<u8> {
    let internal_request = InternalSubmitMultiRequest {
        sequence_number: request.sequence_number,
        service_type: request.service_type.clone(),
        source_addr_ton: request.source_addr_ton.into(),
        source_addr_npi: request.source_addr_npi.into(),
        source_addr: request.source_addr.clone(),
        destinations: request
            .destinations
            .iter()
            .map(|d| d.clone().into())
            .collect(),
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
        optional_params: request.tlvs.iter().map(|t| t.clone().into()).collect(),
    };
    let mut buffer = Vec::new();
    internal_request
        .encode(&mut buffer)
        .expect("Failed to encode SubmitMultiRequest");
    buffer
}

/// Decodes a byte buffer into a `SubmitMultiRequest`.
///
/// # Errors
///
/// Returns an error string if the decoding fails.
#[uniffi::export]
pub fn decode_submit_multi_request(buffer: &[u8]) -> Result<SubmitMultiRequest, String> {
    match InternalSubmitMultiRequest::decode(buffer) {
        Ok(internal_resp) => Ok(SubmitMultiRequest {
            sequence_number: internal_resp.sequence_number,
            service_type: internal_resp.service_type,
            source_addr_ton: internal_resp.source_addr_ton.into(),
            source_addr_npi: internal_resp.source_addr_npi.into(),
            source_addr: internal_resp.source_addr,
            destinations: internal_resp
                .destinations
                .into_iter()
                .map(|d| d.into())
                .collect(),
            esm_class: internal_resp.esm_class,
            protocol_id: internal_resp.protocol_id,
            priority_flag: internal_resp.priority_flag,
            schedule_delivery_time: if internal_resp.schedule_delivery_time.is_empty() {
                None
            } else {
                Some(internal_resp.schedule_delivery_time)
            },
            validity_period: if internal_resp.validity_period.is_empty() {
                None
            } else {
                Some(internal_resp.validity_period)
            },
            registered_delivery: internal_resp.registered_delivery,
            replace_if_present_flag: internal_resp.replace_if_present_flag,
            data_coding: internal_resp.data_coding,
            sm_default_msg_id: internal_resp.sm_default_msg_id,
            short_message: internal_resp.short_message,
            tlvs: internal_resp
                .optional_params
                .into_iter()
                .map(|t| t.into())
                .collect(),
        }),
        Err(e) => Err(e.to_string()),
    }
}

/// Encodes a `SubmitMultiResponse` into a byte vector.
///
/// # Panics
///
/// Panics if the internal encoding fails.
#[uniffi::export]
pub fn encode_submit_multi_response(response: &SubmitMultiResponse) -> Vec<u8> {
    let internal_response = InternalSubmitMultiResponse {
        sequence_number: response.sequence_number,
        command_status: response.command_status,
        message_id: response.message_id.clone(),
        status_description: response.status_description.clone(),
        unsuccess_smes: response
            .unsuccess_smes
            .iter()
            .map(|u| u.clone().into())
            .collect(),
    };
    let mut buffer = Vec::new();
    internal_response
        .encode(&mut buffer)
        .expect("Failed to encode SubmitMultiResponse");
    buffer
}

/// Decodes a byte buffer into a `SubmitMultiResponse`.
///
/// # Errors
///
/// Returns an error string if the decoding fails.
#[uniffi::export]
pub fn decode_submit_multi_response(buffer: &[u8]) -> Result<SubmitMultiResponse, String> {
    match InternalSubmitMultiResponse::decode(buffer) {
        Ok(internal_resp) => Ok(SubmitMultiResponse {
            sequence_number: internal_resp.sequence_number,
            command_status: internal_resp.command_status,
            message_id: internal_resp.message_id,
            status_description: internal_resp.status_description,
            unsuccess_smes: internal_resp
                .unsuccess_smes
                .into_iter()
                .map(|u| u.into())
                .collect(),
        }),
        Err(e) => Err(e.to_string()),
    }
}
